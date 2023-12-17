<script lang="ts">
	import { Button, Dropdown, DropdownItem } from 'flowbite-svelte';
	import { createEventDispatcher } from 'svelte';
	import { slide } from 'svelte/transition';

	export let type: 'v' | 'c' | undefined = undefined;
	export let open = false;

  const dispatch = createEventDispatcher<{ change: "v" | "c"}>();

	const changeTypeFactory = (t: 'v' | 'c') => {
		return () => {
			type = t;
			open = false;
      dispatch("change", t)
		};
	};
</script>

<Button color="alternative" size="xs" class="w-20 p-2 rounded-md">
	{#if type === 'v'}
		Verse
	{:else}
		Chorus
	{/if}
</Button>
<Dropdown bind:open class="p-1" transition={slide} shadow border>
	<DropdownItem class="rounded-t transition-all" on:click={changeTypeFactory('v')}>
		Verse
	</DropdownItem>
	<DropdownItem
		class="border-t rounded-b transition-all border-black/10 dark:border-white/10"
		on:click={changeTypeFactory('c')}
	>
		Chorus
	</DropdownItem>
</Dropdown>
