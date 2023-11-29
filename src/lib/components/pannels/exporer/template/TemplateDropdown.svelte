<script lang="ts">
	import type { TemplateModel } from '$lib/models/template';
	import { CirclePlusSolid, EllipseVerticalSolid, PenSolid } from 'flowbite-svelte-icons';
	import { Dropdown, DropdownItem } from 'flowbite-svelte';
	import { fade } from 'svelte/transition';
	import { DefaultTemplateForm, TemplateForm } from '.';

	export let template: TemplateModel;

	let editOpen = false;
	let defaultOpen = false;

	function handleEdit() {
		editOpen = !editOpen;
		document.body.click();
	}

	function handleDefault() {
		defaultOpen = !defaultOpen;
		document.body.click();
	}
</script>

<button
	type="button"
	class="absolute top-1 right-1 border bg-surface-dark/50 text-white border-white/20 p-1.5 hover:bg-white/20 transition-all rounded-full"
>
	<EllipseVerticalSolid class="w-3 h-3" />
</button>
<Dropdown placement="right" transition={fade}>
	<DropdownItem on:click={handleEdit} class="flex items-center gap-3">
		<PenSolid class="w-3 h-3" /> Edit
	</DropdownItem>
	<DropdownItem on:click={handleDefault} class="flex items-center gap-3">
		<CirclePlusSolid class="w-3 h-3" /> Set Default
	</DropdownItem>
</Dropdown>
<TemplateForm
	bind:templateId={template.id}
	bind:templateName={template.name}
	bind:fontSize={template.font_size}
	bind:fontColor={template.font_color}
	bind:fontStyle={template.font_style}
	bind:fontWeight={template.font_weight}
	bind:background={template.background}
	bind:textAlignment={template.text_alignment}
	bind:verticalAlignment={template.vertical_alignment}
	bind:horizontalAlignment={template.horizontal_alignment}
	bind:sideTextFontSize={template.side_text_font_size}
	bind:sideTextFontColor={template.side_text_font_color}
	bind:sideTextFontStyle={template.side_text_font_style}
	bind:sideTextFontWeight={template.side_text_font_weight}
	bind:sideTextTextAlignment={template.side_text_text_alignment}
	bind:sideTextVerticalAlignment={template.side_text_vertical_alignment}
	bind:sideTextHorizontalAlignment={template.side_text_horizontal_alignment}
	bind:open={editOpen}
/>
<DefaultTemplateForm {template} bind:open={defaultOpen} />
