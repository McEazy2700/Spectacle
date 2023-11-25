<script lang="ts">
	import type { AlignmentType } from '$lib/types';
	import { Button, Dropdown, DropdownItem } from 'flowbite-svelte';
	import { fade } from 'svelte/transition';

	export let alignment: AlignmentType = 'start';

	const alignments: AlignmentType[] = [
		'normal',
		'start',
		'center',
		'end',
		'safe',
		'unset',
		'unsafe',
		'stretch',
		'flex-start',
		'flex-end',
		'baseline',
		'initial',
		'inherit'
	];
	let open = false;
</script>

<Button color="alternative" class="rounded w-fit" size="xs">{alignment}</Button>
<Dropdown bind:open transition={fade}>
	{#each alignments as val}
		<DropdownItem
			on:click={() => {
				alignment = val;
				open = false;
			}}>{val}</DropdownItem
		>
	{/each}
</Dropdown>
