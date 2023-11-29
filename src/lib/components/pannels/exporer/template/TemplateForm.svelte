<script lang="ts">
	import { Button, Heading, Input, Label, Modal, Spinner } from 'flowbite-svelte';
	import { scale } from 'svelte/transition';
	import type { AlignmentType, FontStyleType } from '$lib/types';
	import { TemplatePreview, TemplateSettings } from '.';
	import { invoke } from '@tauri-apps/api';
	import type { TemplateOptions } from '$lib/models/template';
	import alerts from '$lib/stores/alerts';
	import { createEventDispatcher } from 'svelte';

	export let open: boolean;
	export let templateName = '';
	export let templateId: number | undefined = undefined;
	export let fontWeight = 400;
	export let fontSize = 30;
	export let fontColor = '#ffffff';
	export let fontStyle: FontStyleType = 'normal';
	export let horizontalAlignment: AlignmentType = 'start';
	export let verticalAlignment: AlignmentType = 'start';
	export let textAlignment: AlignmentType = 'start';
  export let textShadow: boolean = false;
  export let textShadowBlur: number = 1;
  export let textShadowColor: string = "#E2510C"
  export let textShadowVertical: number = 2;
  export let textShadowHorizontal: number = 2;
	export let background: string | undefined = undefined;

	export let sideTextFontWeight = 400;
	export let sideTextFontSize = 30;
	export let sideTextFontColor = fontColor;
	export let sideTextFontStyle: FontStyleType = 'normal';
  export let sideTextShadow: boolean = false;
  export let sideTextShadowBlur: number = 1;
  export let sideTextShadowColor: string = "#E2510C"
  export let sideTextShadowVertical: number = 2;
  export let sideTextShadowHorizontal: number = 2;
	export let sideTextTextAlignment: AlignmentType = 'start';
	export let sideTextVerticalAlignment: AlignmentType = 'start';
	export let sideTextHorizontalAlignment: AlignmentType = 'start';

	let loading = false;

  const dispatch = createEventDispatcher<{ update: null }>();
	async function saveTemplate() {
		loading = true;
		const opts: TemplateOptions = {
			id: templateId,
			name: templateName,
			background: background,
			font_size: parseInt(fontSize.toString()),
			font_color: fontColor,
			font_style: fontStyle,
			font_weight: parseInt(fontWeight.toString()),
      text_shadow: textShadow,
      text_shadow_blur: parseInt(textShadowBlur.toString()),
      text_shadow_color: textShadowColor,
      text_shadow_vertical: parseInt(textShadowVertical.toString()),
      text_shadow_horizontal: parseInt(textShadowHorizontal.toString()),
			text_alignment: textAlignment,
			vertical_alignment: verticalAlignment,
			horizontal_alignment: horizontalAlignment,
			side_text_font_size: parseInt(sideTextFontSize.toString()),
			side_text_font_style: sideTextFontStyle,
			side_text_font_color: sideTextFontColor,
			side_text_font_weight: parseInt(sideTextFontWeight.toString()),
      side_text_shadow: sideTextShadow,
      side_text_shadow_blur: parseInt(sideTextShadowBlur.toString()),
      side_text_shadow_color: sideTextShadowColor,
      side_text_shadow_vertical: parseInt(sideTextShadowVertical.toString()),
      side_text_shadow_horizontal: parseInt(sideTextShadowHorizontal.toString()),
			side_text_text_alignment: sideTextTextAlignment,
			side_text_vertical_alignment: sideTextVerticalAlignment,
			side_text_horizontal_alignment: sideTextHorizontalAlignment
		};
		try {
			await invoke('save_template', { opts });
			loading = false;
			open = false;
			alerts.add({
				message: `Success: ${templateName} ${templateId ? 'Saved' : 'Created'}`,
				kind: 'success'
			});
      dispatch("update")
		} catch (err) {
			loading = false;
			alerts.add({ message: `Error: ${JSON.stringify(err)}`, kind: 'error' });
		}
	}
</script>

<Modal size="lg" transition={scale} bind:open>
	<form on:submit|preventDefault={saveTemplate} class="text-start flex flex-col items-start">
		<Heading tag="h4">
			{#if templateId}
				Edit: {templateName}
			{:else}
				New Template
			{/if}
		</Heading>
		<div class="mt-5 flex w-full flex-col gap-1">
			<Label class="font-medium">Template name</Label>
			<Input
				bind:value={templateName}
				class="rounded text-xl font-semibold w-full"
				placeholder="Scripture Template"
				size="sm"
				type="text"
			/>
		</div>
		<TemplatePreview
			bind:fontSize
			bind:fontColor
			bind:fontStyle
			bind:fontWeight
			bind:background
      bind:textShadow
      bind:textShadowBlur
      bind:textShadowColor
      bind:textShadowVertical
      bind:textShadowHorizontal
			bind:textAlignment
			bind:verticalAlignment
			bind:horizontalAlignment
			bind:sideTextFontSize
			bind:sideTextFontColor
			bind:sideTextFontStyle
			bind:sideTextFontWeight
      bind:sideTextShadow
      bind:sideTextShadowBlur
      bind:sideTextShadowColor
      bind:sideTextShadowVertical
      bind:sideTextShadowHorizontal
			bind:sideTextTextAlignment
			bind:sideTextVerticalAlignment
			bind:sideTextHorizontalAlignment
		/>
		<TemplateSettings
			name="Main Styles"
			bind:fontStyle
			bind:fontColor
			bind:fontSize
			bind:fontWeight
			bind:background
      bind:textShadow
      bind:textShadowBlur
      bind:textShadowColor
      bind:textShadowVertical
      bind:textShadowHorizontal
			bind:textAlignment
			bind:verticalAlignment
			bind:horizontalAlignment
		/>
		<TemplateSettings
			noBackground
			name="Side Text Styles"
			bind:fontSize={sideTextFontSize}
			bind:fontColor={sideTextFontColor}
			bind:fontStyle={sideTextFontStyle}
			bind:fontWeight={sideTextFontWeight}
      bind:textShadow={sideTextShadow}
      bind:textShadowBlur={sideTextShadowBlur}
      bind:textShadowColor={sideTextShadowColor}
      bind:textShadowVertical={sideTextShadowVertical}
      bind:textShadowHorizontal={sideTextShadowHorizontal}
			bind:textAlignment={sideTextTextAlignment}
			bind:verticalAlignment={sideTextVerticalAlignment}
			bind:horizontalAlignment={sideTextHorizontalAlignment}
		/>
		<Button type="submit" class="self-end">
			{#if loading}
				<Spinner color="white" />
			{:else}
				Save
			{/if}
		</Button>
	</form>
</Modal>
