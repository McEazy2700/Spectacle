<script lang="ts">
	import { Pannel } from '$lib/components/pannels';
	import activePreview from '$lib/stores/active/preview';
	import { Slide, SpectacleSlide } from '../slides';
	import { ActiveSong } from './active';

	let scale: number = 1;
	let width: number;
	let height: number;
	function rescale(e: CustomEvent<HTMLElement>) {
		height = e.detail.clientHeight;
		width = e.detail.clientWidth;
		scale = e.detail.clientWidth / 1265;
	}
</script>

<Pannel on:resize={rescale} resizeable video class="h-full overflow-y-auto p-0.5 pr-1">
	{#if $activePreview === 'Song'}
		<ActiveSong containerHeight={height} containerWidth={width} {scale} />
	{:else}
		<Slide scale={scale - 0.03} scalable size="lg" maxWidth={width}>
			<SpectacleSlide />
		</Slide>
	{/if}
</Pannel>
