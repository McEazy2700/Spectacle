<script lang="ts">
	import { Slide, SlideActions, SongSlideView } from '$lib/components/slides';
	import activeSong, { songEditing } from '$lib/stores/active/song';
	import liveSong from '$lib/stores/live/song';
	import liveView from '$lib/stores/views/liveView';
	import { makeSlide, parseSongXML } from '$lib/utils/song';
	import editing from '$lib/stores/editing';
	import type { TemplateModel } from '$lib/models/template';

	export let containerWidth: number;
	export let scale: number;
	export let templateId: number;
	export let template: TemplateModel | undefined = undefined;

	$: song = parseSongXML($activeSong?.song.lyrics ?? '', $activeSong?.song.title);

	const handleLiveFactory = (id: number) => {
		return () => {
			if ($activeSong) {
				liveSong.set({ song, liveVerseIndex: id, templateId });
				liveView.set('Song');
			}
		};
	};

	const editHandlerFactory = (index: number) => {
		return () => {
			if ($activeSong !== undefined) {
				songEditing.set({
					song: $activeSong.song,
					activeVerseIndex: index,
					lang: $activeSong.lang,
					format: $activeSong.format
				});
				editing.set(true);
			}
		};
	};
</script>

<ul class="grid grid-cols-3 pr-2 gap-3">
	{#if templateId}
		{#each song.verses as verse, id}
			<div class="relative group">
				<Slide
					focused={$liveSong?.song.title == song.title && $liveSong?.liveVerseIndex == id}
					bordered
					maxWidth={(containerWidth - 10) / 3}
					scale={scale * 1.3}
					scalable
					size="xs"
				>
					<SongSlideView {template} preview slide={makeSlide(verse, templateId)} />
				</Slide>
				<SlideActions
					on:live={handleLiveFactory(id)}
					on:edit={editHandlerFactory(id)}
					scale={scale * 1.2}
					slideType="Song"
					slideName={verse.label}
					slide={makeSlide(verse, templateId)}
					slideId={`${verse.label} ${id + 1}`}
					class="absolute opacity-0 group-hover:opacity-100 -top-0 -right-[6px] transition-all"
				/>
			</div>
		{/each}
	{/if}
</ul>
