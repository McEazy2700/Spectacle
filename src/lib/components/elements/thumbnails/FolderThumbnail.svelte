<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { ThumbnailButton } from '../buttons';
	import { FolderSolid } from 'flowbite-svelte-icons';

	export let path: string;
	let folderName = path.split('/')[path.split('/').length - 2];

	const dispatch = createEventDispatcher<{ select: string }>();

	function handleSelect() {
		dispatch('select', path);
	}
</script>

<ThumbnailButton on:click={handleSelect} name={folderName}>
	<div class="flex items-center justify-center w-full h-full p-3">
		<FolderSolid class="w-24 h-24" />
	</div>
</ThumbnailButton>
