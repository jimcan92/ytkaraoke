<script lang="ts">
	import { files } from '$lib/states/files.svelte';
	import { playing } from '$lib/states/queue.svelte';
	import { search } from '$lib/states/search.svelte';
	import { Download, List, ListMinus, ListPlus, Square } from 'lucide-svelte';
	import { onMount } from 'svelte';

	let isDisplayThumb = false;

	onMount(async () => {
		await files.init();
	});
</script>

<div class="relative flex w-full items-center gap-3 overflow-x-auto p-3">
	{#if files.filtered.length}
		<div class="flex h-full flex-col flex-wrap gap-2">
			{#each files.filtered as videoFile}
				{#if isDisplayThumb}
					<div class="group relative flex aspect-video h-full w-max">
						<img
							src={videoFile.thumbnail}
							alt=""
							class="rounded border border-transparent group-hover:border group-hover:border-base-content"
						/>
						<div
							class="absolute inset-0 z-[2] flex bg-base-300 bg-opacity-0 p-2 opacity-0 group-hover:bg-opacity-80 group-hover:opacity-100"
						>
							<p class="overflow-hidden text-sm">
								{videoFile.title}
							</p>
						</div>
						<button
							onclick={() => {
								// if (inQueue(videoFile.id)) removeFromQueue(videoFile.id);
								// else
								playing.add(videoFile);
								// getVideoFiles();
							}}
							class:hidden={!playing.exists(videoFile.id)}
							class:btn-info={!playing.exists(videoFile.id)}
							class="btn btn-circle btn-accent btn-sm absolute bottom-2 right-2 z-[4] group-hover:inline-flex"
						>
							{#if playing.exists(videoFile.id)}
								<ListMinus class="h-[1-rem] w-[1rem]" />
							{:else}
								<ListPlus class="h-[1-rem] w-[1rem]" />
							{/if}
						</button>
					</div>
				{:else}
					<div class="group relative rounded bg-base-200 px-4 py-2">
						<p class="text-ellipsis text-nowrap">
							{videoFile.title}
						</p>
						<button
							onclick={() => {
								if (playing.exists(videoFile.id)) playing.remove(videoFile.id);
								else playing.add(videoFile);
							}}
							class:hidden={!playing.exists(videoFile.id)}
							class:btn-info={!playing.exists(videoFile.id)}
							class="btn btn-circle btn-accent btn-xs absolute bottom-2 right-2 z-[4] group-hover:inline-flex"
						>
							{#if playing.exists(videoFile.id)}
								<ListMinus class="h-[1-rem] w-[1rem]" />
							{:else}
								<ListPlus class="h-[1-rem] w-[1rem]" />
							{/if}
						</button>
					</div>
				{/if}
			{/each}
		</div>
	{:else if search.videos.length}
		{#each search.videos as video}
			{#if !files.inDownloadQueue(video.id)}
				<div class="group relative flex aspect-video h-full w-max">
					<img
						src={video.thumbnails[0].url}
						alt=""
						class="rounded border border-transparent group-hover:border group-hover:border-base-content"
					/>
					<div
						class="absolute inset-0 z-[2] flex bg-base-300 bg-opacity-0 p-2 opacity-0 group-hover:bg-opacity-80 group-hover:opacity-100"
					>
						<p class="overflow-hidden text-sm">
							{video.title}
						</p>
					</div>
					{#if files.downloading && files.downloading.id === video.id}
						<div
							class="radial-progress absolute bottom-2 right-2 z-[4] bg-base-200/50 text-[0.6rem] text-accent"
							style="--value:{files.dlProgress}; --size:2rem; --thickness: 2px;"
							role="progressbar"
						>
							{files.dlProgress}%
						</div>
					{:else}
						<button
							onclick={() => files.addToDQueue(video)}
							class="btn btn-circle btn-sm absolute bottom-2 right-2 z-[4] hidden group-hover:inline-flex"
						>
							<Download class="h-[1rem] w-[1rem]" />
						</button>
					{/if}
				</div>
			{/if}
		{/each}
	{:else}
		<div class="flex w-full flex-col items-center justify-center text-sm leading-tight">
			<p>No song found. Search in YouTube.</p>
		</div>
	{/if}
	<menu
		class="group join absolute right-2 top-2 z-10 border border-accent opacity-30 hover:opacity-100"
	>
		<li>
			<button
				class="btn join-item btn-sm group-hover:btn-accent"
				onclick={() => (isDisplayThumb = false)}
			>
				<List class="h-4 w-4 text-accent group-hover:text-base-300" />
			</button>
		</li>
		<li>
			<button
				class="btn join-item btn-sm group-hover:btn-accent"
				onclick={() => (isDisplayThumb = true)}
			>
				<Square class="h-4 w-4 text-accent group-hover:text-base-300" />
			</button>
		</li>
	</menu>
</div>
