<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { createEventDispatcher } from 'svelte';
	import { Button, Dropdown, DropdownItem, Heading, Modal, Spinner } from 'flowbite-svelte';
	import { DownloadSolid } from 'flowbite-svelte-icons';
	import { onMount } from 'svelte';
	import { scale, slide } from 'svelte/transition';
	import { BIBLE_VERSIONS, type BibleVersion } from '$lib/constants/bible';
	import alerts from '$lib/stores/alerts';

	const dispatch = createEventDispatcher<{ select: BibleVersion }>();

	type DownloadingItem = { version: string; status: boolean; statusText: string };
	export let activeBibleVersion: string;
	let downloaded: string[] = [];
	let downloading: DownloadingItem | undefined;
	let extractedDir: string | undefined = undefined;

	$: activeBible = BIBLE_VERSIONS.filter((b) => b.version == activeBibleVersion)[0];

	function get_donwloaded_bibles() {
		invoke('get_downloaded_bible_versions')
			.then((res) => {
				downloaded = res as string[];
				activeBible = BIBLE_VERSIONS.filter((bible) => bible.id === downloaded[0].toUpperCase())[0];
			})
			.catch((err) => alerts.add({ message: err.message ?? '', kind: 'error' }));
	}

	onMount(() => {
		get_donwloaded_bibles();
	});

	function handleSetBible(bible: BibleVersion) {
		return async function () {
			if (!downloaded.includes(bible.id)) {
				downloading = { status: true, statusText: 'Downloading zip...', version: bible.version };
				try {
					if (downloading) {
						let res = await invoke('download_bible', { url: bible.url });
						downloading = { ...downloading, statusText: `Extracting ${res}` };
						res = await invoke('extract_bible_zip', { fileName: res });
						extractedDir = res as string;
						downloading = { ...downloading, statusText: `Parsing File` };
						res = await invoke('parse_bible_sql', { folderName: res });
						downloading = { ...downloading, statusText: 'Creating bible database' };
						res = await invoke('create_bible_db', {
							bibleName: `${res}`.split('_')[0],
							tempFolderName: extractedDir
						});
						downloading = { ...downloading, statusText: 'Cleaning things up' };
						res = await invoke('cleanup_temp');
						alerts.add({ message: `Success: ${bible.version} Downloaded`, kind: 'success' });
						get_donwloaded_bibles();
						downloading = undefined;
					}
				} catch (err) {
					if (downloading) {
						downloading = { ...downloading, status: false };
					}
					downloading = undefined;
					alerts.add({ message: (err as any).message ?? '', kind: 'error' });
				}
			}
      document.body.click();
			dispatch('select', bible);
		};
	}
</script>

<div>
	<Button color="alternative" size="xs">{activeBibleVersion}</Button>
	<Dropdown transition={slide}>
		{#each BIBLE_VERSIONS as version}
			<DropdownItem on:click={handleSetBible(version)} class="flex items-center gap-3">
				{version.version}
				{#if !downloaded.includes(version.id)}
					{#if downloading}
						<Spinner />
					{:else}
						<DownloadSolid />
					{/if}
				{/if}
			</DropdownItem>
		{/each}
	</Dropdown>
	<Modal transition={scale} open={downloading?.status}>
		<Heading tag="h4">Downloading {downloading?.version}</Heading>
		<div class="flex items-center gap-3">
			<span>{downloading?.statusText}</span>
			<Spinner />
		</div>
	</Modal>
</div>
