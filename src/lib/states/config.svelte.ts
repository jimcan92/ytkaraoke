import {
	SettingsConfigKey,
	VideoQualityType,
	type SettingsConfig
} from '$lib/types/settings-config';
import { downloadDir } from '@tauri-apps/api/path';
import { Store } from 'tauri-plugin-store-api';

const settingsStore = new Store('.settings.dat');

let _config = $state<SettingsConfig>({
	downloadDirectory: '',
	videoQuality: VideoQualityType.Highest
});

export const config = {
	get downloadDirectory() {
		return _config.downloadDirectory;
	},
	get videoQuality() {
		return _config.videoQuality;
	},
	setDownloadDirectory(dd: string) {
		_config.downloadDirectory = dd;
	},
	setVideoQuality(vq: VideoQualityType) {
		_config.videoQuality = vq;
	},
	async init() {
		let dDir: string | null = null;
		let vQual: VideoQualityType | null = null;

		dDir = await settingsStore.get(SettingsConfigKey.DOWNLOAD_DIRECTORY);
		vQual = await settingsStore.get(SettingsConfigKey.VIDEO_QUALITY);

		if (!dDir) {
			dDir = await downloadDir();
			await settingsStore.set(SettingsConfigKey.DOWNLOAD_DIRECTORY, dDir);
		}

		if (!vQual) {
			vQual = VideoQualityType.Highest;
			await settingsStore.set(SettingsConfigKey.VIDEO_QUALITY, vQual);
		}

		_config.downloadDirectory = dDir;
		_config.videoQuality = vQual;
	}
};

// export const settingsConfig = writable<SettingsConfig>();

// export const loadConfig = async () => {
// 	console.log(settingsStore.path);

// 	let dDir: string | null = null;
// 	let vQual: VideoQualityType | null;

// 	dDir = await settingsStore.get(SettingsConfigKey.DOWNLOAD_DIRECTORY);
// 	vQual = await settingsStore.get(SettingsConfigKey.VIDEO_QUALITY);

// 	if (!dDir) {
// 		dDir = await downloadDir();
// 		await settingsStore.set(SettingsConfigKey.DOWNLOAD_DIRECTORY, dDir);
// 	}

// 	if (!vQual) {
// 		vQual = VideoQualityType.Highest;
// 		await settingsStore.set(SettingsConfigKey.VIDEO_QUALITY, vQual);
// 	}

// 	settingsConfig.set({ downloadDirectory: dDir, videoQuality: vQual });
// };

// export const setDownloadDirectory = async (dd: string) => {
// 	await settingsStore.set(SettingsConfigKey.DOWNLOAD_DIRECTORY, dd);
// 	settingsConfig.update((sc) => ({ ...sc, downloadDirectory: dd }));
// };

// export const setVideoQuality = async (vq: VideoQualityType) => {
// 	await settingsStore.set(SettingsConfigKey.VIDEO_QUALITY, vq);
// 	settingsConfig.update((sc) => ({ ...sc, videoQuality: vq }));
// };
