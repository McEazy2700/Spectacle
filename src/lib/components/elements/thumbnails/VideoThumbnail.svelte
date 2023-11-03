<script lang="ts">
	import { convertFileSrc } from '$lib/utils/media';
	import { ThumbnailButton } from '../buttons';
	import { createEventDispatcher } from 'svelte';

	export let src: string;
	const fileName = src.split('/').pop();

	const dispatch = createEventDispatcher<{ click: string }>();

	function handleClick() {
		dispatch('click', src);
	}
</script>

<ThumbnailButton on:click={handleClick} title={fileName} name={fileName}>
	<img
		loading="lazy"
		src={convertFileSrc(src, { thumbnail: true })}
		class="w-full h-full object-cover"
		alt={fileName}
	/>
</ThumbnailButton>
