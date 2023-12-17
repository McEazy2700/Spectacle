<script lang="ts">
	import { createEventDispatcher, onDestroy, onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import { twMerge } from 'tailwind-merge';

	export let video = false;
	export let resizeable = false;
	let klass = '';
	export { klass as class };
	export let id: string | undefined = undefined;

	let pannelElement: HTMLElement;

	const dispatch = createEventDispatcher<{ resize: HTMLElement }>();

	function handleResize() {
		dispatch('resize', pannelElement);
	}
	let resizeObserver = new ResizeObserver(handleResize);

	onMount(() => {
		if (resizeable) {
			resizeObserver.observe(pannelElement);
		}
	});

	onDestroy(() => {
		if (resizeable) resizeObserver.disconnect();
	});
</script>

<div
	{id}
	transition:fade
	bind:this={pannelElement}
	style={video ? 'aspect-ratio: 16 / 9;' : ''}
	class={twMerge(
		`border-black/10 inline-block dark:border-white/10 border rounded-xl overflow-hidden`,
		klass
	)}
>
	<slot />
</div>
