<script lang="ts">
	import { Slide } from '$lib/components/slides';
	import type { View } from '$lib/models/slide';
	import { onDestroy, onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';

	let view: View | undefined = undefined;
	export let size: 'xs' | 'sm' | 'md' | 'lg' | 'full' = 'full';
	export let bordered = false;
	let unlisten: Promise<UnlistenFn>;

	onMount(async () => {
		let res = await invoke('get_live_view');
		view = res as View;

		unlisten = listen('state-update', (e) => {
			view = e.payload as View;
		});
	});

	onDestroy(() => {
		unlisten.then((f) => f());
	});
</script>

<Slide templateType={view?.type} {bordered} {size} slide={view?.slide} />
