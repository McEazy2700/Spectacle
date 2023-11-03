<script lang="ts">
	import { Button } from 'flowbite-svelte';
	import { Tiptap } from '$lib/components/elements/tiptap';
	import { createEventDispatcher } from 'svelte';

	export let content = '';
	export let buttonName: string;
	const dispatch = createEventDispatcher<{ submit: string }>();

	function handleSubmit() {
		dispatch('submit', content);
	}
</script>

<form
	on:submit|preventDefault={handleSubmit}
	class="flex flex-col gap-2 border p-2 rounded-md border-black/10 dark:border-white/10"
>
	<Tiptap bind:content />
	<Button type="submit" color="green" size="xs" class="w-fit self-end">{buttonName}</Button>
</form>
