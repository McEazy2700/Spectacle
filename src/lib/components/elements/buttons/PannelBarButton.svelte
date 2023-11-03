<script lang="ts">
	import { Button } from 'flowbite-svelte';
	import { twMerge } from 'tailwind-merge';
	import type { Pannel } from '$lib/types';
	import selectPannel from '$lib/stores/pannels/selectPannel';

	let klass = '';
	export { klass as class };
	export let pannel: Pannel;

	function handleClick() {
		selectPannel.set(pannel);
	}
</script>

<Button
	on:click={handleClick}
	type="button"
	color="alternative"
	class={twMerge('rounded-2xl backdrop-blur bg-black/10 transition-all dark:bg-white/10', klass)}
	size="xs"
>
	{pannel}
</Button>
