import { config } from '$lib/states/config.svelte';
import type { Video, VideoFile } from '$lib/types/api';
import { invoke } from '@tauri-apps/api';
import { appWindow } from '@tauri-apps/api/window';
import filenamify from 'filenamify';

export async function downloadVideo(video: Video): Promise<VideoFile> {
	const vf: VideoFile = await invoke('download', {
		videoFile: video,
		fileName: `${filenamify(video.title)}.mp4`
	});

	return vf;
}

export async function getVideoFiles() {
	const dir = config.downloadDirectory;
	console.log(dir);

	const vfs: VideoFile[] = await invoke('get_video_files', { dir });
	console.log(vfs);

	return vfs;
}

export async function youtubeSearch(query: string) {
	const res: Video[] = await invoke('search', { query: `${query} karaoke` });
	return res;
}

export async function exists(id: string) {
	const res: boolean = await invoke('exists', { id });
	return res;
}

export async function toggleFullscreen() {
	const fs = await appWindow.isFullscreen();
	await appWindow.setFullscreen(!fs);
	return appWindow.isFullscreen();
}
