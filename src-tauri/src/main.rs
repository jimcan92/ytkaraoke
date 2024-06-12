// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use database::{add_video_file, get_all, in_db, init_db, VideoFile};
use rusty_ytdl::{
    search::{SearchResult, Video as SearchVideo, YouTube},
    DownloadOptions, Video, VideoOptions, VideoQuality, VideoSearchOptions,
};
use serde::Serialize;
use state::{AppState, ServiceAccess};
use tauri::{AppHandle, Manager, State, Window, Wry};
use tauri_plugin_store::{with_store, StoreCollection};

#[derive(Clone, Serialize)]
struct ProgressPayload {
    id: String,
    progress: u64,
}

#[tauri::command]
async fn get_video_files(dir: String, app: AppHandle) -> Vec<VideoFile> {
    app.db(|db| get_all(dir, db)).unwrap()
}

#[tauri::command]
async fn exists(id: String, app: AppHandle) -> bool {
    let res = app.db(|db| in_db(id, db));

    match res {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[tauri::command]
async fn download(
    video_file: SearchVideo,
    file_name: String,
    window: Window,
    app: AppHandle,
) -> VideoFile {
    let download_dir = fetch_setting(&app, "downloadDirectory".to_string());
    let video_quality = fetch_setting(&app, "videoQuality".to_string());

    //select the video quality enum based on the video quality string
    let video_quality_enum = match video_quality.as_str() {
        "Highest" => VideoQuality::Highest,
        "Lowest" => VideoQuality::Lowest,
        _ => VideoQuality::HighestVideo,
    };

    let video_options = VideoOptions {
        quality: video_quality_enum,
        filter: VideoSearchOptions::VideoAudio,
        download_options: DownloadOptions {
            dl_chunk_size: Some(1024 * 1024 * 1u64),
        },
        ..Default::default()
    };

    let video = Video::new_with_options(&video_file.id, video_options).unwrap();
    let stream = video.stream().await.unwrap();
    let total = stream.content_length();

    let mut total_downloaded = 0;

    while let Some(chunk) = stream.chunk().await.unwrap() {
        let window = window.clone();
        let id = video_file.id.clone();
        let download_event_name = format!("download_progress_{}", id);

        total_downloaded += chunk.len();
        let percentage = (total_downloaded as f64 / total as f64) * 100.0;
        window
            .emit(
                &download_event_name,
                ProgressPayload {
                    id,
                    progress: percentage as u64,
                },
            )
            .unwrap();
    }

    let path = Path::new(&download_dir).join(file_name);
    video.download(&path).await.unwrap();

    let vf = VideoFile::from_video(&video_file, download_dir);

    app.db(|db| add_video_file(&vf, db)).unwrap();

    vf
}

#[tauri::command]
fn show_in_folder(path: String) {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args(["/select,", &path]) // The comma after select is not a typo
            .spawn()
            .unwrap();
    }

    #[cfg(target_os = "linux")]
    {
        if path.contains(",") {
            // see https://gitlab.freedesktop.org/dbus/dbus/-/issues/76
            let new_path = match metadata(&path).unwrap().is_dir() {
                true => path,
                false => {
                    let mut path2 = PathBuf::from(path);
                    path2.pop();
                    path2.into_os_string().into_string().unwrap()
                }
            };
            Command::new("xdg-open").arg(&new_path).spawn().unwrap();
        } else {
            if let Ok(Fork::Child) = daemon(false, false) {
                Command::new("dbus-send")
                    .args([
                        "--session",
                        "--dest=org.freedesktop.FileManager1",
                        "--type=method_call",
                        "/org/freedesktop/FileManager1",
                        "org.freedesktop.FileManager1.ShowItems",
                        format!("array:string:\"file://{path}\"").as_str(),
                        "string:\"\"",
                    ])
                    .spawn()
                    .unwrap();
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").args(["-R", &path]).spawn().unwrap();
    }
}

fn fetch_setting(app: &AppHandle, key: String) -> String {
    let stores = app.state::<StoreCollection<Wry>>();
    let path = PathBuf::from(".settings.dat");

    let store_value = with_store(app.app_handle().to_owned(), stores, path, |store| {
        Ok(store
            .get(key)
            .and_then(|val| val.as_str().map(|s| s.to_string()))
            .expect("No store"))
    })
    .unwrap();
    store_value
}

#[tauri::command]
async fn search(query: &str, app: AppHandle) -> Result<Vec<SearchVideo>, ()> {
    let youtube = YouTube::new().unwrap();

    let query = query.replace(&['\"', '\'', '`'][..], "");
    let res = youtube.search(query, None).await.unwrap();

    let mut video_results: Vec<SearchVideo> = Vec::new();
    for search_result in res {
        if let SearchResult::Video(video) = search_result {
            //exclude live streams
            let exists_in_db = app.db(|db| in_db(video.clone().id, db)).unwrap();

            if video.duration > 0 && !exists_in_db {
                video_results.push(video)
            }
        }
    }

    return Ok(video_results);
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            db: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            download,
            search,
            show_in_folder,
            get_video_files,
            exists,
        ])
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db = init_db(&handle).expect("Database initialize should succeed");
            *app_state.db.lock().unwrap() = Some(db);

            Ok(())
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
