<script lang="ts">
	import type { FontStyleType } from '$lib/types';
	import { Button, Dropdown, DropdownItem } from 'flowbite-svelte';
	import { fade } from 'svelte/transition';

	export let fontStyle: FontStyleType = 'normal';

	const fontStyles: FontStyleType[] = ['italic', 'oblique', 'normal', 'initial', 'inherit', 'unset'];
	let open = false;
</script>

<Button color="alternative" class="rounded w-fit" size="xs">{fontStyle}</Button>
<Dropdown bind:open transition={fade}>
	{#each fontStyles as val}
		<DropdownItem
			on:click={() => {
				fontStyle = val;
				open = false;
			}}>{val}</DropdownItem
		>
	{/each}
</Dropdown>
