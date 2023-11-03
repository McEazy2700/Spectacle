<script lang="ts">
	import { convertFileSrc } from '$lib/utils/media';
	import { createEventDispatcher } from 'svelte';
	import { ThumbnailButton } from '$lib/components/elements/buttons';

	export let src: string;
	let fileName = src.split('/').pop();

	const dispatch = createEventDispatcher<{ click: string }>();

	function handleClick() {
		dispatch('click', src);
	}
</script>

<ThumbnailButton title={fileName} name={fileName} on:click={handleClick}>
	<img
		loading="lazy"
		class="min-w-full rounded aspect-video object-cover"
		src={convertFileSrc(src)}
		alt={fileName}
	/>
</ThumbnailButton>
