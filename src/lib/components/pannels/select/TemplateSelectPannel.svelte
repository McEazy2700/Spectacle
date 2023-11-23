<script lang="ts">
	import { IconButton, TemplateButton } from '$lib/components/elements';
	// import activePannel from '$lib/stores/pannels/activePannel';
	import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';
	import type { TemplateType } from '$lib/types';
	import { PlusSolid } from 'flowbite-svelte-icons';

	let templates: TemplateType[] = [];
	function handleAdd() {
	}

	onMount(() => {
		invoke('get_templates').then((temps) => {
			templates = temps as TemplateType[];
		});
	});
</script>

<div class="w-full flex flex-col p-2 relative">
	<ul class="flex flex-wrap w-full gap">
		{#each templates as template}
			<TemplateButton {template} />
		{/each}
	</ul>
	<IconButton class="self-center" on:click={handleAdd} >
    <PlusSolid />
	</IconButton>
</div>
