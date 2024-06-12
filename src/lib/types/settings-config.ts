export const enum SettingsConfigKey {
	DOWNLOAD_DIRECTORY = 'downloadDirectory',
	VIDEO_QUALITY = 'videoQuality'
}

export interface SettingsConfig {
	downloadDirectory: string;
	videoQuality: VideoQualityType;
}

export enum VideoQualityType {
	Highest = 'Highest',
	Lowest = 'Lowest'
}
