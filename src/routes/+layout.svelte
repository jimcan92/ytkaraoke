<script lang="ts">
	import { page } from '$app/stores';
	import { config } from '$lib/states/config.svelte';
	import { files } from '$lib/states/files.svelte';
	import { playing } from '$lib/states/queue.svelte';
	import { search } from '$lib/states/search.svelte';
	import { status } from '$lib/states/status.svelte';
	import type { VideoFile } from '$lib/types/api';
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

	import { appWindow } from '@tauri-apps/api/window';

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
		if (playing.current) load(playing.current);
		else paused = true;
	});

	// $effect(() => {
	// 	videoPlayer?.play();
	// 	paused = videoPlayer?.paused ?? true;
	// });

	$effect(() => {
		files.filter(search.query);
	});

	onMount(async () => {
		await config.init();
	});

	// Function to toggle fullscreen
	async function toggleFullscreen() {
		try {
			// Call Tauri API to toggle fullscreen
			await appWindow.setFullscreen(!(await appWindow.isFullscreen()));
		} catch (error) {
			console.error('Error toggling fullscreen:', error);
		}
	}

	// Example event listener (e.g., for a keyboard shortcut or button click)
	document.addEventListener('keydown', (event) => {
		// Example: F11 key to toggle fullscreen
		if (event.key === 'F11') {
			toggleFullscreen();
		}
	});
</script>

<div class="flex h-svh flex-col">
	<div
		class:h-[70%]={showControls}
		class="relative flex h-full items-center justify-center bg-black"
	>
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
		{#if playing.current}
			<div
				class="absolute bottom-5 left-5 flex flex-col gap-2 rounded border p-2 opacity-30 hover:opacity-100"
			>
				<p class="flex items-center gap-2 text-sm">
					<Play class="h-4 w-4" />
					CURRENT SONG
				</p>
				<div class="group relative flex aspect-video h-20 w-max">
					<img
						src={playing.current.thumbnail}
						alt=""
						class="rounded border border-transparent group-hover:border-base-content"
					/>
					<div
						class="z- absolute inset-0 flex bg-base-300 bg-opacity-0 p-2 opacity-0 group-hover:bg-opacity-80 group-hover:opacity-100"
					>
						<p class="overflow-hidden text-sm">
							{playing.current.title}
						</p>
					</div>
				</div>
			</div>
		{/if}
		{#if playing.queue.length}
			<div
				class="absolute bottom-5 right-5 flex flex-col gap-2 rounded border p-2 opacity-30 hover:opacity-100"
			>
				<p class="flex items-center gap-2 text-sm">
					<ChevronLast class="h-4 w-4" />
					NEXT SONG
				</p>
				<div class="group relative flex aspect-video h-20 w-max">
					<img
						src={playing.queue[0].thumbnail}
						alt=""
						class="rounded border border-transparent group-hover:border-base-content"
					/>
					<div
						class="z- absolute inset-0 flex bg-base-300 bg-opacity-0 p-2 opacity-0 group-hover:bg-opacity-80 group-hover:opacity-100"
					>
						<p class="overflow-hidden text-sm">
							{playing.queue[0].title}
						</p>
					</div>
				</div>
			</div>
		{/if}
		<div
			class="absolute bottom-0 flex h-12 w-full items-center justify-center gap-8 bg-gradient-to-t from-base-300/40 to-transparent hover:from-base-300/80 hover:to-base-300/50"
		>
			<label class="btn btn-circle btn-ghost swap swap-rotate btn-sm">
				<input
					type="checkbox"
					bind:checked={showControls}
					onclick={() => (showControls = !showControls)}
				/>
				<!-- {#if !controlsOpen}? -->
				<Search class="swap-off h-[1.2rem] w-[1.2rem]" />
				<!-- {:else}? -->
				<X class="swap-on h-[1.2rem] w-[1.2rem]" />
				<!-- {/if}? -->
			</label>
			<button
				class="btn btn-circle btn-ghost btn-sm"
				onclick={() => {
					if (paused) {
						videoPlayer?.play();
						paused = false;
					} else {
						videoPlayer?.pause();
						paused = true;
					}
				}}
			>
				{#if paused}
					<Play class="h-[1.2rem] w-[1.2rem]" />
				{:else}
					<Pause class="h-[1.2rem] w-[1.2rem]" />
				{/if}
			</button>
			<button onclick={playing.clear} class="btn btn-circle btn-ghost btn-sm">
				<Square class="h-[1.2rem] w-[1.2rem]" />
			</button>
			<button onclick={playing.next} class="btn btn-circle btn-ghost btn-sm">
				<ChevronLast class="h-[1.2rem] w-[1.2rem]" />
			</button>
			<button onclick={status.toggleFullscreen} class="btn btn-circle btn-ghost btn-sm">
				{#if status.fullscreen}
					<Minimize class="h-[1.2rem] w-[1.2rem]" />
				{:else}
					<Maximize class="h-[1.2rem] w-[1.2rem]" />
				{/if}
			</button>
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
