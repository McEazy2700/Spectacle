<script lang="ts">
	import schedule from '$lib/stores/schedule';
	import { Slide, NoSlides } from '$lib/components/slides';
	import { Pannel } from '$lib/components/pannels';
</script>

<Pannel class="max-w-xs">
	<ul class="w-screen grid grid-cols-1 gap-2 p-1 max-w-xs">
		{#if $schedule.length < 1}
			<NoSlides />
		{:else}
			{#each $schedule as item}
				<Slide slide={item.slides[0]} />
			{/each}
		{/if}
	</ul>
</Pannel>

<style>
</style>
