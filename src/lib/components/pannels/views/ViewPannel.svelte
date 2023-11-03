<script lang="ts">
	import { Button, Heading } from 'flowbite-svelte';
	import { createEventDispatcher } from 'svelte';
	import { fly } from 'svelte/transition';

	export let disabled = false;
	export let label: string | undefined = undefined;
	const dispatch = createEventDispatcher<{ present: null }>();

	function handlePresent() {
		dispatch('present');
	}
</script>

<div
	transition:fly
	class="relative border border-black/5 dark:border-white/5 shadow-lg overflow-hidden rounded-md w-full"
>
	<div
		class={`
      border-b absolute top-0 flex items-center justify-between p-1 px-2 w-full
      border-black/10 dark:border-white/10 bg-black/10 dark:bg-white/5
    `}
	>
		{#if label !== undefined}
			<Heading class="opacity-60" tag="h6">{label}</Heading>
		{/if}
		<Button
			bind:disabled
			on:click={handlePresent}
			type="button"
			size="xs"
			class="text-[10px] h-fit p-1.5 rounded py-1"
		>
			Present
		</Button>
	</div>
	<div class=" p-3 pt-10 mt-3">
		<slot />
	</div>
</div>
