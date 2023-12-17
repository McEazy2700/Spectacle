<script lang="ts">
	import Plus from '~icons/tabler/plus';
	import Option from '~icons/iwwa/option';
	import { IconButton } from '$lib/components/elements';
	import { EditModeSlideAction, Slide, SongSlideView } from '$lib/components/slides';
	import type { SongData, VerseData } from '$lib/models/song';
	import { songEditing } from '$lib/stores/active/song';
	import { createEventDispatcher } from 'svelte';
	import type { TemplateModel } from '$lib/models/template';
	import { makeSlide } from '$lib/utils/song';

	export let containerHeight: number;
	export let templateId: number;
	export let song: SongData;
  export let template: TemplateModel | undefined = undefined

	const dispatch = createEventDispatcher<{ choseEdit: null }>();

	const editHandlerFactory = (id: number) => {
		return () => {
			songEditing.update((c) => ({ ...c, activeVerseIndex: id }));
			dispatch('choseEdit');
		};
	};

	const addSlide = () => {
		const newVerse: VerseData = {
			type: 'v',
			text: '',
			label: `${song.verses.length + 1}`
		};
		song.verses.push(newVerse);
		song = song;
	};

	const moveUpFactory = (verse: VerseData, index: number) => {
		return () => {
			if (index >= 1) {
				const aboveVerse: VerseData = { ...song.verses[index - 1], label: `${index}` };
				song.verses[index - 1] = verse;
				song.verses[index] = aboveVerse;
				song = song;
			}
		};
	};
</script>

<ul style={`height: ${containerHeight}px;`} class="flex gap-1 p-1 w-fit flex-col overflow-y-auto">
	{#if templateId}
		{#each song.verses as verse, id}
			<button class="relative group" on:click={editHandlerFactory(id)}>
				<Slide
					focused={$songEditing?.activeVerseIndex == id}
					bordered
					maxWidth={180}
					scale={0.55}
					scalable
					size="xs"
				>
					<SongSlideView {template} preview slide={makeSlide(verse, templateId)} />
				</Slide>
				<IconButton
					type="button"
					class="absolute transition-all z-[100] right-1 top-1 p-1 opacity-0 group-hover:opacity-100"
				>
					<Option font-size={15} />
				</IconButton>
				<EditModeSlideAction on:moveUp={moveUpFactory(verse, id)} />
			</button>
		{/each}
	{/if}
	<IconButton on:click={addSlide} type="button" class="w-full" title="Add Slide">
		<Plus font-size={15} />
	</IconButton>
</ul>
