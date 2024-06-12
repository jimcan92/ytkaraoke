import { toggleFullscreen } from '$lib/tauri/commands';

let _busy = $state(false);
let _fullscreen = $state(false);

export const status = {
	get busy() {
		return _busy;
	},
	get fullscreen() {
		return _fullscreen;
	},
	setBusy(value: boolean) {
		_busy = value;
	},
	async toggleFullscreen() {
		_fullscreen = await toggleFullscreen();
	}
};
