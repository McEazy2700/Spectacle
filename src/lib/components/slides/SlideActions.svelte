<script lang="ts">
	import type { SlideType, TemplateView, View } from '$lib/models/slide';
	import { fade } from 'svelte/transition';
	import { Button } from 'flowbite-svelte';
	import { PenSolid, VideoSolid } from 'flowbite-svelte-icons';
	import { twMerge } from 'tailwind-merge';
	import { setLiveView } from '$lib/utils/mutations/live';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher<{ live: null; edit: null }>();

	let klass = '';
	export { klass as class };
	export let scale: number | undefined = undefined;
	export let slide: SlideType;
	export let slideId: string;
	export let slideName: string;
	export let slideType: TemplateView;

	const goLive = () => {
		const view: View = {
			id: slideId,
			name: slideName,
			type: slideType,
			slide
		};
		setLiveView(view, () => dispatch('live'));
	};

	const handleEdit = () => {
		dispatch('edit');
	};
</script>

<div style={`scale: ${scale};`} class={twMerge('flex flex-col', klass)} transition:fade>
	<Button
		color="red"
		title="Go Live"
		on:click={goLive}
		size="xs"
		class="rounded rounded-b-none border border-white/10"
	>
		<VideoSolid size="xs" />
	</Button>
	<Button
		title="Edit"
		on:click={handleEdit}
		size="xs"
		class="rounded border border-white/10 border-t-0 rounded-t-none"
	>
		<PenSolid size="xs" />
	</Button>
</div>
