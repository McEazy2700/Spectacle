<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { IconButton } from '$lib/components/elements';
	import { fly } from 'svelte/transition';

	export let content: string;
	export let slideId: number;
	const dispatch = createEventDispatcher<{ edit: { id: number } }>();

	function handleEdit() {
		dispatch('edit', {
			id: slideId
		});
	}
</script>

<div
	transition:fly
	class={`
    text-black w-full relative aspect-video dark:text-white
    border border-black/10 dark:border-white/10 p-4 rounded
  `}
>
	<IconButton class="absolute top-1 right-1" on:click={handleEdit} variant="edit" />
	<div class="w-[1080] origin-top-left aspect-video">
		<!-- eslint-disable-next-line svelte/no-at-html-tags -->
		{@html content}
	</div>
</div>
