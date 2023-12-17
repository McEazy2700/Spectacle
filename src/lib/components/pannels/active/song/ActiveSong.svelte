<script lang="ts">
	import { getDefaultTemplate, getTemplates } from '$lib/utils/queries/template';
	import { onMount } from 'svelte';
	import editing from '$lib/stores/editing';
	import { EditSong, SongSlides } from '.';
	import type { TemplateModel } from '$lib/models/template';

	export let containerHeight: number;
	export let containerWidth: number;
	export let scale: number;
	let templateId: number;
	let template: TemplateModel | undefined = undefined;

	onMount(() => {
		getDefaultTemplate('Song', (t) => (templateId = t));
	});

	$: {
		if (templateId) {
			getTemplates(templateId, (t) => (template = t));
		}
	}
</script>

<div>
	{#if $editing}
		<EditSong {template} {templateId} {containerHeight} />
	{:else}
		<SongSlides {template} {containerWidth} {scale} {templateId} />
	{/if}
</div>
