<script lang="ts">
	let { text } = $props<{ text: string }>();
</script>

<div class="mask-gradient inline-flex w-full overflow-hidden">
	<p class="inline-block translate-x-full animate-scroll whitespace-nowrap">
		<span class="mx-6">{text}</span>
	</p>
</div>

<style>
	.mask-gradient {
		mask-image: linear-gradient(
			to right,
			transparent 0,
			black 50px,
			black calc(100% - 50px),
			transparent 100%
		);
	}
</style>
