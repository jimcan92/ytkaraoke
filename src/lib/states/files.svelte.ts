import { downloadVideo, getVideoFiles } from '$lib/tauri/commands';
import { Event, type DownloadProgressEvent, type Video, type VideoFile } from '$lib/types/api';
import { listen } from '@tauri-apps/api/event';
import { playing } from './queue.svelte';
import { search } from './search.svelte';

let _downloaded = $state<VideoFile[]>([]);
let _filtered = $state<VideoFile[]>([]);
let _downloadQueue = $state<Video[]>([]);
let _downloading = $state<Video>();
let _dlProgress = $state(0);

async function download(v: Video) {
	_dlProgress = 0;
	let unListen = () => {};

	if (_downloading) {
		unListen = await listen(`${Event.DOWNLOAD_PROGRESS}${_downloading.id}`, (event) => {
			const { progress } = event.payload as DownloadProgressEvent;
			_dlProgress = Math.round(progress);
		});
	}

	const vf = await downloadVideo(v);
	playing.add(vf);
	await loadFiles();
	files.filter(search.query);

	if (_downloadQueue.length) {
		_downloading = _downloadQueue.shift();
		if (_downloading) await download(_downloading);
	} else {
		_downloading = undefined;
		unListen();
	}
}

async function loadFiles() {
	const vfs = await getVideoFiles();
	_downloaded = vfs;
}

export const files = {
	get dlProgress() {
		return _dlProgress;
	},
	get downloaded() {
		return _downloaded;
	},
	get filtered() {
		return _filtered;
	},
	get downloading() {
		return _downloading;
	},
	get downloadQueue() {
		return _downloadQueue;
	},
	async init() {
		await loadFiles();
		this.filter('');
	},
	async addToDQueue(video: Video) {
		if (!_downloadQueue.length && !_downloading) {
			_downloading = video;

			await download(_downloading);
		} else {
			_downloadQueue = [..._downloadQueue, video];
		}
	},
	filter(q: string) {
		if (q) {
			_filtered = _downloaded.filter((d) => d.title.toLowerCase().includes(q));
		} else {
			_filtered = _downloaded;
		}
	},
	inDownloadQueue(id: string) {
		if (_downloading) {
			const dq = [..._downloadQueue, _downloading];
			return dq.map((i) => i.id).includes(id);
		}
	}
};
