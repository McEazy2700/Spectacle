<script lang="ts">
	import { FolderButton } from '$lib/components/elements';
	import { FilePicker } from '$lib/components/forms';
	import { createEventDispatcher } from 'svelte';

	export let size: 'normal' | 'small' = 'normal';

	let mediaType: 'Image' | 'Video' | 'Audio' = 'Image';

	const dispatch = createEventDispatcher<{ select: string }>();

	function handleSelect(e: CustomEvent<string>) {
		dispatch('select', e.detail);
	}
</script>

<div class="w-full flex flex-col h-full relative">
	<div class="flex w-full h-full">
		<div class="flex h-full flex-col">
			<FolderButton
				active={mediaType === 'Image'}
				on:click={() => (mediaType = 'Image')}
				class="border-b-0 rounded-br-none">Images</FolderButton
			>
			<FolderButton
				active={mediaType === 'Video'}
				on:click={() => (mediaType = 'Video')}
				class="rounded-r-none">Videos</FolderButton
			>
			<FolderButton
				active={mediaType === 'Audio'}
				on:click={() => (mediaType = 'Audio')}
				class="rounded-tr-none border-t-0">Audio</FolderButton
			>
		</div>
		<div class="flex-1 min-h-full">
			{#if mediaType === 'Image'}
				<FilePicker on:select={handleSelect} bind:size fileType="Image" />
			{:else if mediaType === 'Video'}
				<FilePicker on:select={handleSelect} bind:size fileType="Video" />
			{:else if mediaType === 'Audio'}
				<FilePicker on:select={handleSelect} bind:size fileType="Audio" />
			{/if}
		</div>
	</div>
</div>
