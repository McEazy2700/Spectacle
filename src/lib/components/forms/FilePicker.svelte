<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { imageExtensions, videoExtensions } from '$lib/constants/extensions';
	import {
		ImageThumbnail,
		FolderThumbnail,
		SpectacleLogo,
		VideoThumbnail,
		SpectacleSpinner,
		IconButton
	} from '../elements';
	import { createEventDispatcher } from 'svelte';
	import { ArrowLeftSolid } from 'flowbite-svelte-icons';
	import { fade } from 'svelte/transition';

	export let fileType: 'Image' | 'Audio' | 'Video';
	export let size: 'normal' | 'small' = 'normal';

	let loading = true;
	let directories: string[] = [];
	let files: string[] = [];
	let navHistory: string[] = [];

	const dispatch = createEventDispatcher<{ select: string }>();

	invoke('scan_media_dir', { mediaType: fileType }).then((res) => {
		loadEntries(res as string[]);
		loading = false;
		navHistory = [...navHistory, 'root'];
	});

	function loadEntries(entries: string[]) {
		let fileExtension = '_';
		for (const path of entries) {
			if (path.endsWith('/') || path.endsWith('\\')) {
				directories = [...directories, path];
			} else {
				fileExtension = path.split('/').pop()?.split('.').pop() ?? '_';
				if (fileType === 'Image') {
					if (imageExtensions.includes(fileExtension)) {
						files = [...files, path];
					}
				} else if (fileType === 'Video') {
					if (videoExtensions.includes(fileExtension)) {
						files = [...files, path];
					}
				}
			}
		}
	}

	function handleFolderNavigate(e: CustomEvent<string>) {
		loading = true;
		const subDir = e.detail.slice(0, -1);
		directories = [];
		files = [];
		invoke('scan_media_dir', { mediaType: fileType, subDir }).then((res) => {
			loadEntries(res as string[]);
			navHistory = [...navHistory, subDir];
			loading = false;
		});
	}

	function handleBackNavigate() {
		const previous = navHistory[navHistory.length - 2];
		loading = true;
		directories = [];
		files = [];
		const subDir = previous === 'root' ? undefined : previous;
		invoke('scan_media_dir', { mediaType: fileType, subDir }).then((res) => {
			loadEntries(res as string[]);
			navHistory.pop();
			navHistory = navHistory;
			loading = false;
		});
	}

	function handleSelct(e: CustomEvent<string>) {
		dispatch('select', e.detail);
	}
</script>

<div
  transition:fade
	role="dialog"
	class="relative border border-gray-400/10 w-full h-full min-h-[40vh] overflow-auto rounded-lg ml-0.5 p-1 z-0"
>
	<IconButton
		on:click={handleBackNavigate}
		disabled={navHistory.length < 2}
		class="absolute p-0.5 transition-all hover:scale-125 px-1 left-0 top-0 backdrop-blur"
	>
		<ArrowLeftSolid />
	</IconButton>
	{#if loading}
		<div class="w-full h-full flex items-center justify-center">
			<SpectacleSpinner />
		</div>
	{:else if files.length < 1 && directories.length < 1}
		<div class="w-full h-full flex items-center justify-center">
			<SpectacleLogo />
		</div>
	{:else}
		<ul
			class={`${
				size === 'small'
					? 'grid-cols-4'
					: 'grid-cols-3 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6'
			}
        grid w-full overflow-x-hidden
      `}
		>
			{#each directories as path}
				<FolderThumbnail {path} on:select={handleFolderNavigate} />
			{/each}
			{#if fileType === 'Image'}
				{#each files as file}
					<ImageThumbnail on:click={handleSelct} src={file} />
				{/each}
			{:else if fileType === 'Video'}
				{#each files as file}
					<VideoThumbnail on:click={handleSelct} src={file} />
				{/each}
			{/if}
		</ul>
	{/if}
</div>
