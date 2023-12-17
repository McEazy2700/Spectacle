<script lang="ts">
	import type { SongSlide } from '$lib/models/slide';
	import type { TemplateModel } from '$lib/models/template';
	import { getTemplates } from '$lib/utils/queries/template';
	import { TemplateWrapper } from '../templates';

	export let slide: SongSlide;
	export let preview = false;
	export let template: TemplateModel | undefined = undefined;

	$: {
		if (template === undefined) {
			getTemplates(slide.Song.template_id, (t) => (template = t));
		}
	}
</script>

<TemplateWrapper {preview} bind:template>
	<svelte:fragment slot="main">
		{slide.Song.text.trim()}
	</svelte:fragment>
	<svelte:fragment slot="side">
		{slide.Song.verse}
	</svelte:fragment>
</TemplateWrapper>
