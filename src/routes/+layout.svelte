<script lang="ts">
	import { page } from '$app/stores';
	import ScrollingText from '$lib/components/scrolling-text.svelte';
	import { config } from '$lib/states/config.svelte';
	import { files } from '$lib/states/files.svelte';
	import { playing } from '$lib/states/queue.svelte';
	import { search } from '$lib/states/search.svelte';
	import { status } from '$lib/states/status.svelte';
	import type { VideoFile } from '$lib/types/api';
	import { shortcut } from '$lib/utils';
	import { path } from '@tauri-apps/api';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import filenamify from 'filenamify';
	import {
		ChevronLast,
		FolderDown,
		ListMusic,
		Maximize,
		Minimize,
		Pause,
		Play,
		Search,
		Square,
		X
	} from 'lucide-svelte';
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';
	import '../app.css';

	const { children } = $props();

	let videoPlayer: HTMLVideoElement | undefined = $state();
	let paused = $state(true);
	let showControls = $state(false);

	async function load(playing: VideoFile) {
		if (videoPlayer) {
			const vp = await path.join(playing.saved_at, `${filenamify(playing.title)}.mp4`);
			const vs = convertFileSrc(vp);
			videoPlayer.src = vs;
			videoPlayer.play();
			paused = videoPlayer.paused;
		}
	}

	$effect(() => {
		if (playing.current) {
			load(playing.current);
		} else paused = true;
	});

	$effect(() => {
		files.filter(search.query);
	});

	function pauseOrPlay() {
		if (paused) {
			if (playing.current) {
				videoPlayer?.play();
				paused = false;
			}
		} else {
			videoPlayer?.pause();
			paused = true;
		}
	}

	const toggleShowControls = () => (showControls = !showControls);

	onMount(async () => {
		await config.init();
	});
</script>

<div class="flex h-svh flex-col">
	<div class="relative flex h-full items-center justify-center bg-black">
		{#if playing.current}
			<video
				bind:this={videoPlayer}
				poster={playing.current.thumbnail}
				onended={playing.next}
				class="h-full"
			>
				<track kind="captions" />
			</video>
		{:else}
			<p>Add songs.</p>
		{/if}
		<div
			class="absolute bottom-0 grid h-12 w-full grid-cols-[minmax(0,1fr)_360px_minmax(0,1fr)] items-center bg-gradient-to-t from-black/80 to-transparent px-6 hover:from-black hover:to-black"
		>
			<div>
				{#if playing.current}
					<ScrollingText text="Current Song: {playing.current.title}" />
				{/if}
			</div>
			<div class="flex items-center justify-evenly">
				<div class="tooltip" data-tip="Search songs <Ctrl + S>">
					<label class="btn btn-circle btn-ghost swap swap-rotate btn-sm">
						<input
							use:shortcut={{ control: true, code: 'KeyS' }}
							type="checkbox"
							bind:checked={showControls}
							onclick={toggleShowControls}
						/>
						<Search class="swap-off h-[1.2rem] w-[1.2rem]" />
						<X class="swap-on h-[1.2rem] w-[1.2rem]" />
					</label>
				</div>
				<div class="tooltip" data-tip={paused ? 'Play <Ctrl + Space>' : 'Pause <Ctrl + Space>'}>
					<button
						use:shortcut={{ control: true, code: 'Space' }}
						class="btn btn-circle btn-ghost btn-sm"
						onclick={pauseOrPlay}
					>
						{#if paused}
							<Play class="h-[1.2rem] w-[1.2rem]" />
						{:else}
							<Pause class="h-[1.2rem] w-[1.2rem]" />
						{/if}
					</button>
				</div>
				<div class="tooltip" data-tip="Stop <Ctrl + Shift + Space>">
					<button
						use:shortcut={{ control: true, shift: true, code: 'Space' }}
						onclick={playing.stop}
						class="btn btn-circle btn-ghost btn-sm"
					>
						<Square class="h-[1.2rem] w-[1.2rem]" />
					</button>
				</div>
				<div class="tooltip" data-tip="Next <Ctrl + N>">
					<button
						use:shortcut={{ control: true, code: 'KeyN' }}
						onclick={playing.next}
						class="btn btn-circle btn-ghost btn-sm"
					>
						<ChevronLast class="h-[1.2rem] w-[1.2rem]" />
					</button>
				</div>
				<div class="tooltip" data-tip="Fullscreen <F11>">
					<button
						use:shortcut={{ code: 'F11' }}
						onclick={status.toggleFullscreen}
						class="btn btn-circle btn-ghost btn-sm"
					>
						{#if status.fullscreen}
							<Minimize class="h-[1.2rem] w-[1.2rem]" />
						{:else}
							<Maximize class="h-[1.2rem] w-[1.2rem]" />
						{/if}
					</button>
				</div>
			</div>
			<div>
				{#if playing.queue.length}
					<ScrollingText text="Next Song: {playing.queue[0].title}" />
				{/if}
			</div>
		</div>
	</div>
	{#if showControls}
		<div class="flex h-[30%] w-full" in:slide out:slide>
			<ul class="menu h-full gap-2 bg-base-300 p-4">
				<li>
					{#if $page.url.pathname === '/'}
						<label
							class:input-error={search.query && !files.filtered.length}
							class="input input-sm input-bordered flex w-52 items-center gap-2 pr-1"
						>
							{#if status.busy}
								<span class="loading loading-spinner loading-xs"></span>
							{:else}
								<Search class="h-4 w-4 opacity-70" />
							{/if}
							<input
								bind:value={search.query}
								placeholder="Search Title or Singer"
								type="text"
								class="min-w-0 grow"
								onkeypress={(e) => {
									if (e.key === 'Enter') {
										search.onSearch();
									}
								}}
							/>
							{#if search.query}
								<button class="btn btn-circle btn-ghost btn-xs" onclick={search.clear}>
									<X class="h-4 w-4" />
								</button>
							{/if}
						</label>
					{:else}
						<a href="/">
							<Search class="h-4 w-4" />
							Search
						</a>
					{/if}
				</li>
				<li>
					<a href="/queue" class:active={$page.url.pathname === '/queue'}>
						<ListMusic class="h-4 w-4" />
						Queue
					</a>
				</li>
				<li>
					<a href="/downloading" class:active={$page.url.pathname === '/folder'}>
						<FolderDown class="h-4 w-4" />
						Downloading
					</a>
				</li>
			</ul>
			{@render children()}
		</div>
	{/if}
</div>
