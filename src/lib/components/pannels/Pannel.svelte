<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import { twMerge } from 'tailwind-merge';

	export let video = false;
	let klass = '';
	export { klass as class };
	export let id: string | undefined = undefined;

	let pannelElement: HTMLElement;

	function handleVideo() {
		pannelElement.style.width = `${((pannelElement.clientHeight * 16) / 9).toFixed(1)}px`;
	}
	let observer = new MutationObserver(handleVideo);

	onMount(() => {
		if (video) {
			handleVideo();
			observer.observe(document.body, { childList: true, subtree: true });
		}
	});

	onDestroy(() => {
		if (video) observer.disconnect();
	});
</script>

<div
	{id}
	transition:fade
	bind:this={pannelElement}
	class={twMerge(
		`border-black/10 max-w-full max-h-full inline-block dark:border-white/10 border rounded-xl w-full
      overflow-hidden`,
		klass
	)}
>
	<slot />
</div>
