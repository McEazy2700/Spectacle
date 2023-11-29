<script lang="ts">
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { emit } from '@tauri-apps/api/event';
	import Icon from '@iconify/svelte';
	import { Button, Span } from 'flowbite-svelte';
	import presenting from '$lib/stores/presenting';

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
	on:click={handlePresent}
	color={$presenting ? 'green' : 'primary'}
	size="xs"
	class="flex flex-col items-center"
>
	<Icon
		class={`${$presenting ? 'animate-pulse text-red-500' : ''}`}
		icon="fluent:record-24-regular"
	/>
	<Span class="text-[10px] text-white">{$presenting ? 'Live' : 'Go Live'}</Span>
</Button>
