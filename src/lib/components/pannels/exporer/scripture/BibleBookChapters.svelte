<script>
	import activeScripture from '$lib/stores/activeScripture';
	import { Button, Label } from 'flowbite-svelte';
	import { fade } from 'svelte/transition';

	$: {
		if ($activeScripture.chapter > $activeScripture.book.chapters) {
			activeScripture.update((curr) => ({ ...curr, chapter: 1 }));
		}
	}

	$: activeBookChapters = Array.from({ length: $activeScripture.book.chapters }, (_, i) => i).map(
		(i) => i + 1
	);
</script>

<div class="flex flex-col items-center rounded-lg overflow-hidden">
	<Label class="font-semibold p-1 px-2 bg-white/20 w-full">Chapter</Label>
	<ul
		transition:fade
		class={`
          grid grid-cols-4 max-w-fit p-1 border border-black/10 dark:border-white/10
          rounded-b-lg max-h-[50vh] overflow-auto gap-1
        `}
	>
		{#each activeBookChapters as chapter}
			<Button
				class="aspect-square"
				on:click={() => activeScripture.update((curr) => ({ ...curr, chapter }))}
				size="xs"
				color={$activeScripture.chapter === chapter ? 'primary' : 'alternative'}
			>
				{chapter}
			</Button>
		{/each}
	</ul>
</div>
