use std::fs;

use rusqlite::{named_params, Connection, Error};
use rusty_ytdl::{search::Video, Thumbnail};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

const CURRENT_DB_VERSION: u32 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoFile {
    pub id: String,
    pub title: String,
    pub url: String,
    pub thumbnail: String,
    pub saved_at: String,
}

impl VideoFile {
    pub fn from_video(video: &Video, saved_at: String) -> Self {
        let video = video.clone();
        let thumbnail: &Thumbnail = video.thumbnails.first().unwrap();
        VideoFile {
            id: video.id,
            title: video.title,
            url: video.url,
            thumbnail: thumbnail.clone().url,
            saved_at,
        }
    }
}

pub fn init_db(app_handle: &AppHandle) -> Result<Connection, Error> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("The app data directory should exist.");
    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let sqlite_path = app_dir.join("data.db");

    println!("{:?}", sqlite_path.as_path());

    let mut db = Connection::open(sqlite_path)?;

    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| Ok(row.get(0)?))?;
    drop(user_pragma);

    update_db_if_needed(&mut db, existing_user_version)?;

    Ok(db)
}

pub fn update_db_if_needed(db: &mut Connection, existing_user_version: u32) -> Result<(), Error> {
    if existing_user_version < CURRENT_DB_VERSION {
        db.pragma_update(None, "journal_mode", "WAL")?;

        let tx = db.transaction()?;

        tx.pragma_update(None, "user_version", CURRENT_DB_VERSION)?;

        tx.execute_batch(
            "
        CREATE TABLE videofiles (
            id TEXT PRIMARY KEY,
            url TEXT NOT NULL,
            title TEXT NOT NULL,
            thumbnail TEXT,
            savedAt TEXT
        )",
        )?;

        tx.commit()?;
    }

    Ok(())
}

pub fn get_all(dir: String, db: &Connection) -> Result<Vec<VideoFile>, Error> {
    let mut statement = db.prepare("SELECT * FROM videofiles WHERE savedAt=?")?;
    let mut rows = statement.query([dir])?;
    let mut items = Vec::new();

    while let Some(row) = rows.next()? {
        let category = VideoFile {
            id: row.get("id")?,
            url: row.get("url")?,
            title: row.get("title")?,
            thumbnail: row.get("thumbnail")?,
            saved_at: row.get("savedAt")?,
        };

        items.push(category);
    }

    Ok(items)
}

pub fn add_video_file(file: &VideoFile, db: &Connection) -> Result<(), Error> {
    let mut statement = db.prepare(
        "INSERT INTO videofiles (id, url, title, thumbnail, savedAt) VALUES (@id, @url, @title, @thumbnail, @savedAt)",
    )?;
    statement.execute(named_params! {"@id": file.id, "@url": file.url, "@title": file.title, "@thumbnail": file.thumbnail, "@savedAt": file.saved_at})?;

    Ok(())
}

pub fn in_db(id: String, db: &Connection) -> Result<bool, Error> {
    db.prepare("SELECT * FROM videofiles WHERE id=?")?
        .exists([id])
    // statement.exists([id])
}
