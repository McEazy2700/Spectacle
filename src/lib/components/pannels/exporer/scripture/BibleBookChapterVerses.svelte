<script lang="ts">
	import { SpectacleLogo } from '$lib/components/elements';
	import scriptures from '$lib/stores/lists/scriptures';
	import { Label } from 'flowbite-svelte';
	import { fade } from 'svelte/transition';
	import { ScriptureText } from '.';
	import { onMount } from 'svelte';
	import { getDefaultTemplate } from '$lib/utils/queries/template';

	let templateId: number;

	onMount(async () => {
		getDefaultTemplate('Scripture', (t) => (templateId = t));
	});
</script>

<div class="flex flex-col items-center rounded-lg flex-1 overflow-hidden">
	<Label class="font-semibold p-1 px-2 bg-black/10 dark:bg-white/20 w-full">Verse</Label>
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
			{#each $scriptures as scripture}
				<ScriptureText {templateId} {scripture} />
			{/each}
		{/if}
	</ul>
</div>
