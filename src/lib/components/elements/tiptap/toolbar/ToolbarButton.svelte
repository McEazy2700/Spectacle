<script lang="ts">
	import { Button, Popover } from 'flowbite-svelte';
	import { createEventDispatcher } from 'svelte';
	import { fade } from 'svelte/transition';
	import { twMerge } from 'tailwind-merge';

	let klass = '';

	export { klass as class };
	export let disabled = false;
	export let shortCut: string | undefined = undefined;
	export let title: string | undefined = undefined;
	export let active = false;

	const dispatch = createEventDispatcher<{ click: HTMLButtonElement }>();

	function handleClick(e: MouseEvent) {
		dispatch('click', e.currentTarget as HTMLButtonElement);
	}
</script>

<div>
	<Button
		class={twMerge(
			`border border-black/10 dark:border-white/10 rounded m-0 p-1 ${
				active ? 'bg-black/10 dark:bg-white/10' : ''
			}`,
			klass
		)}
		{disabled}
		on:click={handleClick}
		type="button"
		color="alternative"
		size="xs"
	>
		<slot />
	</Button>
	{#if shortCut || title}
		<Popover class="flex text-xs items-center gap-2" transition={fade}>
			{title}
			{#if shortCut}
				<code class="text-xs bg-black/10 p-1 rounded">{shortCut}</code>
			{/if}
		</Popover>
	{/if}
</div>
