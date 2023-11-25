<script lang="ts">
	import { Button, Heading, Input, Label, Modal, Spinner } from 'flowbite-svelte';
	import { scale } from 'svelte/transition';
	import type { AlignmentType, FontStyleType } from '$lib/types';
	import { TemplatePreview, TemplateSettings } from '.';
	import { invoke } from '@tauri-apps/api';
	import type { TemplateOptions } from '$lib/models/template';
	import alerts from '$lib/stores/alerts';

	export let open: boolean;
	export let templateName = '';
	export let templateId: number | undefined = undefined;
	export let fontWeight = 400;
	export let fontSize = 30;
	export let fontColor = '';
	export let fontStyle: FontStyleType = 'normal';
	export let horizontalAlignment: AlignmentType = 'start';
	export let verticalAlignment: AlignmentType = 'start';
	export let textAlignment: AlignmentType = 'start';
	export let background: string | undefined = undefined;

	export let sideTextFontWeight = 400;
	export let sideTextFontSize = 30;
	export let sideTextFontColor = '';
	export let sideTextFontStyle: FontStyleType = 'normal';
	export let sideTextHorizontalAlignment: AlignmentType = 'start';
	export let sideTextVerticalAlignment: AlignmentType = 'start';
	export let sideTextTextAlignment: AlignmentType = 'start';

	let loading = false;

	async function saveTemplate() {
		loading = true;
		const opts: TemplateOptions = {
			id: templateId,
			name: templateName,
			font_size: parseInt(fontSize.toString()),
			font_color: fontColor,
			font_style: fontStyle,
			font_weight: parseInt(fontWeight.toString()),
			background: background,
			text_alignment: textAlignment,
			side_text_font_size: parseInt(sideTextFontSize.toString()),
			side_text_font_style: sideTextFontStyle,
			side_text_font_color: sideTextFontColor,
			vertical_alignment: verticalAlignment,
			side_text_font_weight: parseInt(sideTextFontWeight.toString()),
			horizontal_alignment: horizontalAlignment,
			side_text_text_alignment: sideTextTextAlignment,
			side_text_vertical_alignment: sideTextVerticalAlignment,
			side_text_horizontal_alignment: sideTextHorizontalAlignment
		};
		try {
			await invoke('save_template', { opts });
			loading = false;
			open = false;
			alerts.add({ message: `Success: ${templateName} Template created`, kind: 'success' });
		} catch (err) {
			loading = false;
			alerts.add({ message: `Error: ${(err as any).message}`, kind: 'error' });
		}
	}
</script>

<Modal size="lg" transition={scale} bind:open>
	<form on:submit|preventDefault={saveTemplate} class="flex flex-col">
		<Heading tag="h4">New Template</Heading>
		<div class="mt-5 flex flex-col gap-1">
			<Label class="font-medium">Template name</Label>
			<Input
				bind:value={templateName}
				class="rounded"
				placeholder="Scripture Template"
				size="sm"
				type="text"
			/>
		</div>
		<TemplatePreview
			bind:fontStyle
			bind:fontColor
			bind:fontSize
			bind:fontWeight
			bind:background
			bind:textAlignment
			bind:verticalAlignment
			bind:horizontalAlignment
			bind:sideTextFontSize
			bind:sideTextFontColor
			bind:sideTextFontStyle
			bind:sideTextFontWeight
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
			bind:horizontalAlignment
			bind:verticalAlignment
			bind:textAlignment
		/>
		<TemplateSettings
			noBackground
			name="Side Text Styles"
			bind:fontStyle={sideTextFontStyle}
			bind:fontColor={sideTextFontColor}
			bind:fontSize={sideTextFontSize}
			bind:fontWeight={sideTextFontWeight}
			bind:horizontalAlignment={sideTextHorizontalAlignment}
			bind:verticalAlignment={sideTextVerticalAlignment}
			bind:textAlignment={sideTextTextAlignment}
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
