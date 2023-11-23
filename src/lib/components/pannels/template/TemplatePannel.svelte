<script lang="ts">
	import { TemplateWraper } from '$lib/components/templates';
	import type { FontWeight, TemplateType } from '$lib/types';
	import { TemplateSettings } from '.';

	// let templateId: number | undefined = $activeTemplate.id;
	// let templateName: string | undefined = $activeTemplate.name;
	// let font: string | undefined = $activeTemplate.font;
	// let fontSize: number | undefined = $activeTemplate.font_size;
	// let fontWeight: FontWeight | undefined = $activeTemplate.font_weight;
	// let backgroundType: 'Video' | 'Image' = $activeTemplate.background_type ?? 'Image';
	// let backgroundURL: string | undefined = $activeTemplate.background_url;

	let template: TemplateType;

	// $: {
	// 	template = {
	// 		font,
	// 		id: templateId,
	// 		name: templateName,
	// 		font_size: fontSize,
	// 		font_weight: fontWeight,
	// 		background_type: backgroundType,
	// 		background_url: backgroundURL
	// 	};
	// }
</script>

<div class="border gap-3 p-2 flex w-full border-black/10 dark:border-white/10 rounded">
	<div class="flex-1 w-full aspect-video">
		<TemplateWraper bind:template>Text</TemplateWraper>
	</div>
	<!-- <TemplateSettings -->
	<!-- 	bind:fontSize -->
	<!-- 	bind:fontWeight -->
	<!-- 	bind:templateName -->
	<!-- 	bind:templateId -->
	<!-- 	bind:backgroundURL -->
	<!-- 	bind:backgroundType -->
	<!-- /> -->
</div>
