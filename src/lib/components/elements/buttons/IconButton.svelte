<script lang="ts">
	import { Button } from 'flowbite-svelte';
	import { createEventDispatcher } from 'svelte';
	import { twMerge } from 'tailwind-merge';

	let klass = '';
	export { klass as class };
	export let title: string | undefined = undefined;
	export let circle = false;
	export let disabled = false;
	export let active = false;
  export let type: "reset" | "button" | "submit" = "button"
	const dispatch = createEventDispatcher<{ click: MouseEvent }>();

	function handleClick(e: MouseEvent) {
		dispatch('click', e);
	}
</script>

<Button
  {type}
	bind:disabled
	on:click={handleClick}
	{title}
	class={twMerge(
		`w-fit self-end rounded bg-black/10 dark:bg-white/10 border-black/20
      dark:border-white/20 transition-all
    ${circle ? 'rounded-full aspect-square' : ''} ${active ? '!bg-red-700' : ''}`,
		klass
	)}
	size="xs"
>
	<slot />
</Button>
