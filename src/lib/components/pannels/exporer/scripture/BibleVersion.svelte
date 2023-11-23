<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { Button, Dropdown, DropdownItem, Heading, Modal, Spinner } from 'flowbite-svelte';
	import { DownloadSolid } from 'flowbite-svelte-icons';
	import { onMount } from 'svelte';
	import { scale, slide } from 'svelte/transition';
	import { BIBLE_VERSIONS } from '$lib/constants/bible';

	type DownloadingItem = { version: string; status: boolean; statusText: string };
	let active = '';
	let downloaded: string[] = [];
	let downloading: DownloadingItem | undefined;
	let extractedDir: string | undefined = undefined;

	function get_donwloaded_bibles() {
		invoke('get_downloaded_bible_versions')
			.then((res) => {
				downloaded = res as string[];
				active = downloaded[0].toUpperCase();
			})
			.catch((err) => console.log(err));
	}

	onMount(() => {
		get_donwloaded_bibles();
	});

	function handleSetBible({ version, url }: { version: string; url: string }) {
		return async function () {
			if (!downloaded.includes(version)) {
				downloading = { status: true, statusText: 'Downloading zip...', version };
				try {
					if (downloading) {
						let res = await invoke('download_bible', { url });
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
						get_donwloaded_bibles();
						downloading = undefined;
					}
				} catch (err) {
					if (downloading) {
						downloading = { ...downloading, status: false };
					}
          downloading = undefined;
					console.log(err);
				}
			}
		};
	}
</script>

<div>
	<Button color="alternative" size="xs">{active}</Button>
	<Dropdown transition={slide}>
		{#each BIBLE_VERSIONS as version}
			<DropdownItem
				on:click={handleSetBible({ version: version.id, url: version.url })}
				class="flex items-center gap-3"
			>
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
