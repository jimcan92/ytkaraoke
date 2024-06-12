<script lang="ts">
	import { playing } from '$lib/states/queue.svelte';
	import { ListMinus } from 'lucide-svelte';
</script>

<div class="flex h-full w-full items-center gap-3 overflow-x-auto p-3">
	{#if playing.queue.length}
		{#each playing.queue as queuedVideo}
			<div class="group relative flex aspect-video h-full w-max">
				<img
					src={queuedVideo.thumbnail}
					alt=""
					class="rounded border border-transparent group-hover:border group-hover:border-base-content"
				/>
				<div
					class="absolute inset-0 z-[2] flex bg-base-300 bg-opacity-0 p-2 opacity-0 group-hover:bg-opacity-80 group-hover:opacity-100"
				>
					<p class="overflow-hidden text-sm">
						{queuedVideo.title}
					</p>
				</div>
				<button
					class="btn btn-circle btn-accent btn-sm absolute bottom-2 right-2 z-[4] hidden group-hover:inline-flex"
					on:click={() => playing.remove(queuedVideo.id)}
				>
					<ListMinus class="h-[1-rem] w-[1rem]" />
				</button>
			</div>
		{/each}
	{:else}
		<div class="flex w-full items-center justify-center">
			<p>Add songs</p>
		</div>
	{/if}
</div>
