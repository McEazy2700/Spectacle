<script lang="ts">
	import type { TemplateModel } from '$lib/models/template';
	import alerts from '$lib/stores/alerts';
	import { invoke } from '@tauri-apps/api';
	import { Button, Heading, Modal } from 'flowbite-svelte';

	let templateViews = ['Song', 'Text', 'Media', 'Sermon', 'Scripture'];
	export let open: boolean;
	export let template: TemplateModel;

	function handlerFactory(view: string) {
		return async function () {
			try {
				await invoke('set_default_template', { opts: { view, template_id: template.id } });
				alerts.add({
					message: `Success: ${template.name} set as ${view} default template`,
					kind: 'success'
				});
				open = false;
			} catch (err) {
				alerts.add({ message: `Error: ${(err as any).message}`, kind: 'error' });
				open = false;
			}
		};
	}
</script>

<Modal bind:open>
	<Heading tag="h4">View Type</Heading>
	<ul class="grid grid-cols-3 gap-3">
		{#each templateViews as view}
			<Button
				on:click={handlerFactory(view)}
				color="alternative"
				class="aspect-video w-full max-w-[15rem] text-xl"
			>
				{view}
			</Button>
		{/each}
	</ul>
</Modal>
