<script>
	import { files } from '$lib/states/files.svelte';
</script>

{#if files.downloading}
	<div class="flex grow">
		<div class="flex w-full max-w-60 flex-col gap-2 p-2">
			<p class="font-bold italic">Downloading</p>
			<div class="group relative flex aspect-video h-max w-full">
				<img
					src={files.downloading.thumbnails[0].url}
					alt=""
					class="rounded border border-transparent group-hover:border group-hover:border-base-content"
				/>
				<div
					class="absolute inset-0 z-[2] flex bg-base-300 bg-opacity-0 p-2 opacity-0 group-hover:bg-opacity-80 group-hover:opacity-100"
				>
					<p class="overflow-hidden text-sm">
						{files.downloading.title}
					</p>
				</div>
				<div class="absolute inset-0 z-[4] flex items-center justify-center bg-base-200/50">
					<div
						class="radial-progress text-accent"
						style="--value:{files.dlProgress};"
						role="progressbar"
					>
						{files.dlProgress}%
					</div>
				</div>
			</div>
		</div>
		{#if files.downloadQueue.length}
			<div class="flex flex-1 flex-col gap-2 p-2">
				<p class="font-bold italic">Waiting</p>
				<div class="flex h-full flex-col flex-wrap gap-2 overflow-x-auto">
					{#each files.downloadQueue as dq}
						<div class="w-max max-w-80 text-ellipsis rounded bg-base-200 px-2 py-1">
							<p class=" overflow-hidden truncate">{dq.title}</p>
						</div>
					{/each}
				</div>
			</div>
		{/if}
	</div>
{:else}
	<div class="flex w-full items-center justify-center">
		<p>Nothing to show</p>
	</div>
{/if}
