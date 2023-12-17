<script lang="ts">
	import type { ScriptureSlide } from '$lib/models/slide';
	import type { TemplateModel } from '$lib/models/template';
	import { getTemplates } from '$lib/utils/queries/template';
	import { TemplateWrapper } from '../templates';

	export let slide: ScriptureSlide;
	export let template: TemplateModel | undefined = undefined;
  export let preview = false

	$: {
    if (template === undefined) {
      getTemplates(slide.Scripture.template_id, (t) => (template = t));
    }
  }
</script>

<TemplateWrapper {preview} bind:template>
	<svelte:fragment slot="main">
		{slide.Scripture.text}
	</svelte:fragment>
	<svelte:fragment slot="side">
		{slide.Scripture.passage}
	</svelte:fragment>
</TemplateWrapper>
