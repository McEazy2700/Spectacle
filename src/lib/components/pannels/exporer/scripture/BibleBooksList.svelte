<script>
	import { Button, Label } from 'flowbite-svelte';
	import { BIBLE_BOOKS } from '$lib/constants/bible';
	import activeScripture from '$lib/stores/active/scripture';
</script>

<div class="flex flex-col items-center rounded-lg overflow-hidden">
	<Label class="font-semibold p-1 px-2 bg-black/10 dark:bg-white/20 w-full">Book</Label>
	<ul
		class={`
            max-w-fit p-1 border border-black/10 dark:border-white/10 rounded-b-lg
            max-h-[50vh] overflow-auto gap-1 grid
          `}
	>
		{#each BIBLE_BOOKS as book}
			<li>
				<Button
					on:click={() => activeScripture.update((curr) => ({ ...curr, book: book }))}
					color={$activeScripture?.book?.name == book.name ? 'primary' : 'alternative'}
					class="rounded w-full"
				>
					{book.name}
				</Button>
			</li>
		{/each}
	</ul>
</div>
