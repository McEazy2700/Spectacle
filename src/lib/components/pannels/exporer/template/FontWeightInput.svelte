<script lang="ts">
	import { Button, Dropdown, DropdownItem } from 'flowbite-svelte';
	import { fade } from 'svelte/transition';

	export let weight: number = 100;

	const weights = [100, 200, 300, 400, 500, 600, 700, 800, 900];
	let open = false;
</script>

<Button color="alternative" class="rounded w-fit" size="xs">{weight}</Button>
<Dropdown bind:open transition={fade}>
	{#each weights as val}
		<DropdownItem
			on:click={() => {
				weight = val;
				open = false;
			}}>{val}</DropdownItem
		>
	{/each}
</Dropdown>
