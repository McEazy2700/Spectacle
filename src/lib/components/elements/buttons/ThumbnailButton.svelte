<script lang="ts">
	import { P } from 'flowbite-svelte';
	import { createEventDispatcher } from 'svelte';
	import { fly } from 'svelte/transition';

	const dispatch = createEventDispatcher<{ click: MouseEvent }>();
	export let name: string | undefined = undefined;
	export let title: string | undefined = undefined;

	function handleClick(e: MouseEvent) {
		dispatch('click', e);
	}
</script>

<button
	{title}
	transition:fly
	on:click={handleClick}
	class={`
    p-0.5 border rounded-md hover:bg-gray-400/50 transition-all gap-1
    border-black/10 dark:border-white/5 m-1 duration-300 flex flex-col
    bg-black/10 items-center justify-between
  `}
>
	<slot />
	{#if name}
		<P
			class={`
      font-medium text-xs max-w-full rounded self-center whitespace-nowrap
      flex text-center p-0.5 items-center justify-center bg-black/10 dark:bg-white/10
    `}
		>
			<span class="text-ellipsis overflow-hidden">
				{name}
			</span>
		</P>
	{/if}
</button>
