import { youtubeSearch } from '$lib/tauri/commands';
import type { Video } from '$lib/types/api';
import { status } from './status.svelte';

let _searchedVideos = $state<Video[]>([]);
let _query = $state('');

export const search = {
	get query() {
		return _query;
	},
	get videos() {
		return _searchedVideos;
	},
	set query(value: string) {
		_query = value;
	},
	async onSearch() {
		status.setBusy(true);
		const svs = await youtubeSearch(_query);
		_searchedVideos = svs;
		status.setBusy(false);
	},
	clear() {
		_query = '';
		_searchedVideos = [];
	}
};
