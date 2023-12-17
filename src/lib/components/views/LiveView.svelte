<script lang="ts">
	import { ScriptureSlideView, Slide, SongSlideView, SpectacleSlide } from '$lib/components/slides';
	import type { ScriptureSlide, SongSlide, View } from '$lib/models/slide';
	import { onDestroy, onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import type { TemplateModel } from '$lib/models/template';
	import { getTemplates } from '$lib/utils/queries/template';

	let view: View | undefined = undefined;
	export let size: 'xs' | 'sm' | 'md' | 'lg' | 'full' = 'full';
  export let scale: number | undefined = undefined
  export let scalable = false
	export let bordered = false;
	let unlisten: Promise<UnlistenFn>;

	let scriptureSlide: ScriptureSlide | undefined = undefined;
	let songSlide: SongSlide | undefined = undefined;
	let template: TemplateModel | undefined = undefined;
	let templateId: number | undefined = undefined;

	onMount(async () => {
		let res = await invoke('get_live_view');
		view = res as View;

		unlisten = listen('state-update', (e) => {
			view = e.payload as View;
		});
	});

	onDestroy(() => {
		unlisten.then((f) => f());
	});

	$: {
		if (view?.type === 'Scripture') {
			const slide = view.slide as ScriptureSlide;
			scriptureSlide = slide;
			templateId = slide.Scripture.template_id;
		} else if (view?.type === 'Song') {
			const slide = view.slide as SongSlide;
			songSlide = slide;
			templateId = slide.Song.template_id;
		}
	}

	$: {
		if (templateId !== undefined) getTemplates(templateId, (t) => (template = t));
	}
</script>

<Slide {scalable} {scale} {bordered} {size}>
	{#if view?.type === 'Scripture' && scriptureSlide}
		<ScriptureSlideView {template} slide={scriptureSlide} />
	{:else if view?.type === 'Song' && songSlide}
		<SongSlideView {template} slide={songSlide} />
	{:else}
		<SpectacleSlide />
	{/if}
</Slide>
