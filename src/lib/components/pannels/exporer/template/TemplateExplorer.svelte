<script lang="ts">
	import { Button } from 'flowbite-svelte';
	import { PlusSolid } from 'flowbite-svelte-icons';
	import { TemplateDropdown, TemplateForm, TemplatePreview } from '.';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api';
	import type { TemplateModel } from '$lib/models/template';

	let open = false;
	let templates: TemplateModel[] = [];

  async function updateTemplates() {
		templates = (await invoke('get_templates')) as any;
  }
	onMount(updateTemplates);
</script>

<div class="flex flex-col">
	<ul>
		{#each templates as template}
			<div
				class="max-w-xs relative transition-all rounded-lg border border-black/20 dark:border-white/20 p-0 w-full grid overflow-hidden"
			>
				<TemplatePreview
					size="xs"
					bind:fontSize={template.font_size}
					bind:fontColor={template.font_color}
					bind:fontStyle={template.font_style}
					bind:fontWeight={template.font_weight}
					bind:background={template.background}
          bind:textShadow={template.text_shadow}
          bind:textShadowBlur={template.text_shadow_blur}
          bind:textShadowColor={template.text_shadow_color}
          bind:textShadowVertical={template.text_shadow_vertical}
          bind:textShadowHorizontal={template.text_shadow_horizontal}
					bind:textAlignment={template.text_alignment}
					bind:verticalAlignment={template.vertical_alignment}
					bind:horizontalAlignment={template.horizontal_alignment}
					bind:sideTextFontSize={template.side_text_font_size}
					bind:sideTextFontColor={template.side_text_font_color}
					bind:sideTextFontStyle={template.side_text_font_style}
					bind:sideTextFontWeight={template.side_text_font_weight}
          bind:sideTextShadow={template.side_text_shadow}
          bind:sideTextShadowBlur={template.side_text_shadow_blur}
          bind:sideTextShadowColor={template.side_text_shadow_color}
          bind:sideTextShadowVertical={template.side_text_shadow_vertical}
          bind:sideTextShadowHorizontal={template.side_text_shadow_horizontal}
					bind:sideTextTextAlignment={template.side_text_text_alignment}
					bind:sideTextVerticalAlignment={template.side_text_vertical_alignment}
					bind:sideTextHorizontalAlignment={template.side_text_horizontal_alignment}
				/>
				<span
					class="p-0.5 font-medium rounded mt-1 bg-black/20 flex items-center justify-center dark:bg-white/20"
				>
					{template.name}
				</span>
				<TemplateDropdown {template} />
			</div>
		{/each}
	</ul>
	<Button
		type="button"
		on:click={() => (open = !open)}
		color="alternative"
		size="xs"
		class="w-fit self-center"
	>
		<PlusSolid class="w-4 h-4" />
	</Button>
</div>
<TemplateForm on:update={updateTemplates} bind:open />
