<script lang="ts">
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { emit } from '@tauri-apps/api/event';
	import { Button, Span } from 'flowbite-svelte';
	import presenting from '$lib/stores/presenting';
	import VideoIcon from '~icons/eva/video-fill';
	import CloseIcon from '~icons/ep/close-bold';

	let presentationWindow: WebviewWindow;

	function handlePresent() {
		if ($presenting && presentationWindow) {
			presentationWindow.close();
			presenting.set(false);
			emit('live-change', {
				status: false
			});
		} else {
			presentationWindow = new WebviewWindow('presentation-window', {
				url: '/presentation',
				closable: false,
				fullscreen: true
			});
			presenting.set(true);
			emit('live-change', {
				status: true
			});
		}
	}
</script>

<Button
	title={$presenting ? 'Stop presenting' : 'Go Live'}
	on:click={handlePresent}
	color={$presenting ? 'red' : 'green'}
	size="xs"
	class="flex flex-col items-center rounded-md p-1 gap-1"
>
	{#if $presenting}
		<CloseIcon font-size={12} />
	{:else}
		<VideoIcon font-size={12} />
	{/if}
	<span class="text-xs bg-black/20 rounded-sm px-2">
		{#if $presenting}
			Close
		{:else}
			Go Live
		{/if}
	</span>
</Button>
