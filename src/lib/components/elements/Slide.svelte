<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	export let html: string | undefined = undefined;
	export let slideId: number;
	export let active = false;

	const dispatch = createEventDispatcher<{ click: { id: number; html?: string } }>();

	function handleClick() {
		dispatch('click', {
			id: slideId,
			html
		});
	}
</script>

<button
	on:click={handleClick}
	class={`
      aspect-video max-w-[200px] border rounded border-black/10
      dark:border-white/10 w-screen justify-center p-1
      ${active ? 'outline outline-black/20 dark:outline-white/20 shadow-lg' : ''}
    `}
>
	<slot />
	{#if html}
		<div class="w-[720px] aspect-video scale-[0.25] origin-top-left">
			<!-- eslint-disable-next-line svelte/no-at-html-tags -->
			{@html html}
		</div>
	{/if}
</button>
