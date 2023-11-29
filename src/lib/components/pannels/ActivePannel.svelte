<script lang="ts">
	import { Pannel, SermonViewPannel } from '$lib/components/pannels';
	import activePannel from '$lib/stores/pannels/activePannel';
	import { Slide } from '../slides';

	let scale: number = 1;
	function rescale(e: CustomEvent<HTMLElement>) {
		scale = e.detail.clientWidth / 1265;
	}
</script>

<Pannel on:resize={rescale} video class="aspect-video h-full relative">
	<div class="absolute w-full h-full top-0 left-0">
		{#if $activePannel}
			<div class="p-1 flex-1">
				<SermonViewPannel />
			</div>
		{:else}
			<Slide bind:scale size="lg" />
		{/if}
	</div>
</Pannel>
