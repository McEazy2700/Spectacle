<script lang="ts">
	import schedule from '$lib/stores/schedule';
	import { Slide, NoSlides } from '$lib/components/slides';
	import { Pannel } from '$lib/components/pannels';
</script>

<Pannel class="max-w-[15rem] w-full">
	<ul class="w-full grid grid-cols-1 gap-2 p-1 max-w-xs">
		{#if $schedule.length < 1}
			<NoSlides />
		{:else}
			{#each $schedule as item}
				<Slide slide={item.slide} />
			{/each}
		{/if}
	</ul>
</Pannel>

<style>
</style>
