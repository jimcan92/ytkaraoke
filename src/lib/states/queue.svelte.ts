import type { VideoFile } from '$lib/types/api';

type Queue = {
	queue: VideoFile[];
	playing?: VideoFile;
};

let _queue = $state<Queue>({ queue: [] });

export const playing = {
	get queue() {
		return _queue.queue;
	},
	get current() {
		return _queue.playing;
	},
	add(vf: VideoFile) {
		if (!_queue.queue.length && !_queue.playing) {
			_queue.playing = vf;
			return;
		}
		_queue.queue = [..._queue.queue, vf];
	},
	remove(id: string) {
		_queue.queue = _queue.queue.filter((q) => q.id !== id);
	},
	next() {
		const fvf = _queue.queue.shift();
		if (fvf) _queue.playing = fvf;
	},
	stop() {
		_queue.queue = [];
		_queue.playing = undefined;
	},
	exists(id: string) {
		return this.queue.map((q) => q.id).includes(id);
	}
};
