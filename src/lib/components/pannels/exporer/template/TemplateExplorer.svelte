<script lang="ts">
	import { Button } from 'flowbite-svelte';
	import { PlusSolid } from 'flowbite-svelte-icons';
	import { TemplateForm, TemplatePreview } from '.';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api';
	import type { TemplateModel } from '$lib/models/template';

	let open = false;
	let templates: TemplateModel[] = [];

	onMount(async () => {
		templates = (await invoke('get_templates')) as any;
	});

	$: console.log(templates);
</script>

<div class="flex flex-col">
	<ul>
		{#each templates as template}
			<Button color="alternative" class="max-w-xs p-0 grid overflow-hidden">
				<TemplatePreview
          size="xs"
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
				/>
        <span class="p-2 dark:bg-white/20">{template.name}</span>
			</Button>
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
<TemplateForm bind:open />
