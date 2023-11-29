<script lang="ts">
	import { createEventDispatcher, onDestroy, onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import { twMerge } from 'tailwind-merge';

	export let video = false;
	let klass = '';
	export { klass as class };
	export let id: string | undefined = undefined;

	let pannelElement: HTMLElement;

	const dispatch = createEventDispatcher<{ resize: HTMLElement }>();

	function handleVideo() {
    if (video) {
      pannelElement.style.width = `${((pannelElement.clientHeight * 16) / 9).toFixed(1)}px`;
    }
	}

  function handleResize() {
    handleVideo();
    dispatch("resize", pannelElement);
  }
	let mutationObserver = new MutationObserver(handleVideo);
  let resizeObserver = new ResizeObserver(handleResize);

	onMount(() => {
		if (video) {
			handleVideo();
			mutationObserver.observe(document.body, { childList: true, subtree: true });
      resizeObserver.observe(pannelElement);
		}
	});

	onDestroy(() => {
		if (video) mutationObserver.disconnect();
    resizeObserver.disconnect()
	});
</script>

<div
	{id}
	transition:fade
	bind:this={pannelElement}
	class={twMerge(
		`border-black/10 inline-block dark:border-white/10 border rounded-xl overflow-hidden`,
		klass
	)}
>
	<slot />
</div>
