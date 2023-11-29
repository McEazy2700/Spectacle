<script lang="ts">
	import { SpectacleLogo } from '$lib/components/elements';
	import scriptures from '$lib/stores/scriptures';
	import { Label } from 'flowbite-svelte';
	import { fade } from 'svelte/transition';
	import { ScriptureText } from '.';
	import { onMount } from 'svelte';
	import alerts from '$lib/stores/alerts';
	import { invoke } from '@tauri-apps/api';

	let templateId: number;

	onMount(async () => {
		try {
			const res = await invoke('get_default_template', { view: 'Scripture' });
			templateId = (res as any).template_id;
		} catch (err) {
			alerts.add({ message: `Error: ${(err as any).message}`, kind: 'error' });
		}
	});
</script>

<div class="flex flex-col items-center rounded-lg flex-1 overflow-hidden">
	<Label class="font-semibold p-1 px-2 bg-white/20 w-full">Verse</Label>
	<ul
		transition:fade
		class={`
          grid max-w-fit p-1 border border-black/10 dark:border-white/10
          rounded-b-lg max-h-[50vh] overflow-auto gap-1
        `}
	>
		{#if $scriptures.length < 1}
			<div class="w-full p-36 px-56">
				<SpectacleLogo />
			</div>
		{:else}
			{#each $scriptures as scripture, index}
				<ScriptureText {index} {templateId} {scripture} />
			{/each}
		{/if}
	</ul>
</div>
