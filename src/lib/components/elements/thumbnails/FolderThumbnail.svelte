<script lang="ts">
	import Icon from '@iconify/svelte';
	import { createEventDispatcher } from 'svelte';
	import { ThumbnailButton } from '../buttons';

	export let path: string;
	let folderName = path.split('/')[path.split('/').length - 2];

	const dispatch = createEventDispatcher<{ select: string }>();

	function handleSelect() {
		dispatch('select', path);
	}
</script>

<ThumbnailButton on:click={handleSelect} name={folderName}>
	<Icon class="w-full aspect-video text-[7rem] text-gray-400" icon="bi:folder-fill" />
</ThumbnailButton>
