<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { twMerge } from 'tailwind-merge';

	let klass = '';
	export { klass as class };
	export let active = false;

	const dispatch = createEventDispatcher<{ click: MouseEvent }>();
	function handleClick(e: MouseEvent) {
		dispatch('click', e);
	}
</script>

<button
	class={twMerge(
		`p-2 border rounded-md rounded-l-none border-white/5 px-6 bg-black/20 hover:bg-black/30 transition-all
    ${active ? 'bg-black/30' : ''}`,
		klass
	)}
	on:click={handleClick}
	type="button"
>
	<slot />
</button>
