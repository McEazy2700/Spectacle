<script lang="ts">
	import { Button, Label, Modal } from 'flowbite-svelte';
	import { FileExplorer } from '..';
	import { PlusSolid } from 'flowbite-svelte-icons';
	import { scale } from 'svelte/transition';
	import { imageExtensions } from '$lib/constants/extensions';
	import { convertFileSrc } from '$lib/utils/media';

	let open = false;
	export let src: string | undefined = undefined;
	export let mediaType: 'Video' | 'Image' | 'Audio' | undefined = undefined;

	$: {
		if (imageExtensions.includes(src?.split('.').pop() ?? '__')) {
			mediaType = 'Image';
		} else {
			mediaType = 'Video';
		}
	}
</script>

<Button
	color="alternative"
	type="button"
	on:click={() => (open = !open)}
	class="aspect-video max-w-[9rem] p-0 overflow-hidden"
>
	{#if !src}
		<div>
			<PlusSolid />
		</div>
	{:else}
		<img
			class="w-full object-cover"
			alt={src.split('/').pop()}
			src={convertFileSrc(src, { thumbnail: mediaType === 'Video' ? true : false })}
		/>
	{/if}
</Button>
<Modal transition={scale} size="xl" bind:open>
	<Label class="font-bold text-lg">Media Picker</Label>
	<FileExplorer
		on:select={(e) => {
			src = e.detail;
			open = !open;
		}}
	/>
</Modal>
