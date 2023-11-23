<script lang="ts">
	import type { SlideType } from '$lib/models/slide';
	import { draggable } from '$lib/actions/dnd';
	import { SpectacleSlide } from '.';
	import { fade } from 'svelte/transition';
	import { onDestroy, onMount } from 'svelte';

	export let slide: SlideType | undefined = undefined;
	export let size: 'xs' | 'sm' | 'md' | 'lg' | 'full' = 'full';
	export let bordered = false;

	let slideElement: HTMLDivElement;
	let containerElement: HTMLDivElement;

	const handleRescale = () => {
		if (slideElement) {
			slideElement.style.scale = (containerElement.clientWidth / 1200).toString();
		}
	};

	let observer = new MutationObserver(handleRescale);

	onMount(() => {
    handleRescale()
		observer.observe(document.body, { childList: true, subtree: true });
	});

	onDestroy(() => {
		observer.disconnect();
	});
</script>

<div
	class={`w-full aspect-video rounded-lg
    ${size === 'xs' ? 'max-w-xs' : size === 'sm' ? 'max-w-sm' : ''}
    ${bordered ? 'border border-black/20 dark:border-white/20' : ''}
  `}
	bind:this={containerElement}
>
	<div bind:this={slideElement} class="aspect-video flex items-center">
		{#if !slide}
			<SpectacleSlide />
		{:else}
			<div use:draggable={slide}>
				<p>Some</p>
			</div>
		{/if}
	</div>
</div>
