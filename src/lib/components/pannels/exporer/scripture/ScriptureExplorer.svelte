<script lang="ts">
	import { Button, Label } from 'flowbite-svelte';
	import { BibleVersion, ScriptureText } from '.';
	import type { ScriptureResult } from '$lib/models/scripture';
	import { BIBLE_BOOKS, BIBLE_VERSIONS } from '$lib/constants/bible';
	import { fade } from 'svelte/transition';
	import { invoke } from '@tauri-apps/api';
	import { SpectacleLogo } from '$lib/components/elements';

	let activeBook = BIBLE_BOOKS[0];
	let activeChapter = 1;
	let scriptures: ScriptureResult[] = [];
	let activeBible = BIBLE_VERSIONS[0];

	$: scriptureOpts = {
		version: activeBible.id,
		book: activeBook.abrv,
		chapter: activeChapter
	};

	$: {
		if (activeChapter > activeBook.chapters) {
			activeChapter = 1;
		}
	}

	$: activeBookChapters = Array.from({ length: activeBook.chapters }, (_, i) => i).map(
		(i) => i + 1
	);

	$: {
		invoke('get_scriptures', { opts: scriptureOpts }).then((res) => {
			scriptures = res as ScriptureResult[];
		});
	}
</script>

<div class="flex min-h-full items-center gap-4">
	<div class="flex flex-col gap-2">
		<div>
			<BibleVersion
				on:select={(e) => (activeBible = e.detail)}
				activeBibleVersion={activeBible.version}
			/>
		</div>
		<div class="flex items-start gap-1">
			<div class="flex flex-col items-center rounded-lg overflow-hidden">
				<Label class="font-semibold p-1 px-2 bg-white/20 w-full">Book</Label>
				<ul
					class={`
            max-w-fit p-1 border border-black/10 dark:border-white/10 rounded-b-lg
            max-h-[50vh] overflow-auto gap-1 grid
          `}
				>
					{#each BIBLE_BOOKS as book}
						<li>
							<Button
								on:click={() => (activeBook = book)}
								color={activeBook.name == book.name ? 'primary' : 'alternative'}
								class="rounded w-full"
							>
								{book.name}
							</Button>
						</li>
					{/each}
				</ul>
			</div>
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
							on:click={() => (activeChapter = chapter)}
							size="xs"
							color={activeChapter === chapter ? 'primary' : 'alternative'}
						>
							{chapter}
						</Button>
					{/each}
				</ul>
			</div>
			<div class="flex flex-col items-center rounded-lg flex-1 overflow-hidden">
				<Label class="font-semibold p-1 px-2 bg-white/20 w-full">Verse</Label>
				<ul
					transition:fade
					class={`
          grid max-w-fit p-1 border border-black/10 dark:border-white/10
          rounded-b-lg max-h-[50vh] overflow-auto gap-1
        `}
				>
					{#if scriptures.length < 1}
						<div class="w-full p-36 px-56">
							<SpectacleLogo />
						</div>
					{:else}
						{#each scriptures as scripture}
							<ScriptureText {scripture} />
						{/each}
					{/if}
				</ul>
			</div>
		</div>
	</div>
</div>
