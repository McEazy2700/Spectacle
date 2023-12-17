<script lang="ts">
	import { Slide, SongSlideView } from '$lib/components/slides';
	import { songEditing } from '$lib/stores/active/song';
	import { makeSlide, parseSongXML } from '$lib/utils/song';
	import { Heading, Modal } from 'flowbite-svelte';
	import { fade, scale } from 'svelte/transition';
	import { EditModeSongSlides, SongEditor } from '.';
	import { onMount } from 'svelte';
	import type { VerseData } from '$lib/models/song';
	import { Reveal } from '$lib/components/reveal';
	import type { TemplateModel } from '$lib/models/template';

	export let containerHeight: number;
	export let templateId: number;
	export let template: TemplateModel | undefined = undefined;

	let title = $songEditing?.song?.title;
	let song = parseSongXML($songEditing?.song?.lyrics ?? '', $songEditing?.song?.title);
	let previewOpen = false;

	let currentVerseText = '';
	let currentVerse: VerseData | undefined = undefined;
	let currentVerseType: 'c' | 'v' | undefined = undefined;

	const updateValue = () => {
		if ($songEditing && song.verses.length > $songEditing.activeVerseIndex) {
			currentVerse = song.verses[$songEditing.activeVerseIndex];
			currentVerseType = currentVerse.type;
			currentVerseText = currentVerse.text;
		}
	};

	onMount(() => updateValue());
</script>

<div transition:fade class="flex">
	<EditModeSongSlides
		{template}
		on:choseEdit={updateValue}
		bind:containerHeight
		bind:song
		bind:templateId
	/>
	<SongEditor
		bind:currentVerseText
		bind:previewOpen
		bind:currentVerseType
		bind:containerHeight
		bind:song
		bind:title
	/>
</div>

<Modal bind:open={previewOpen} size="lg" transition={scale} class="z-0 h-[90vh]">
	<Heading tag="h2">{$songEditing?.song?.title}</Heading>
	<Reveal let:R>
		{#each song.verses as verse}
			<R.Slide>
				<Slide scalable scale={0.631} size="sm">
					<SongSlideView slide={makeSlide(verse, templateId)} />
				</Slide>
			</R.Slide>
		{/each}
	</Reveal>
</Modal>
