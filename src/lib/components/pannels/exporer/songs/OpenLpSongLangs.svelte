<script lang="ts">
	import { OPENLP_SONGS, type OpenLpSongLang } from '$lib/constants/song';
	import alerts from '$lib/stores/alerts';
	import { invoke } from '@tauri-apps/api';
	import { Button, Dropdown, DropdownItem, Heading, Modal, Spinner } from 'flowbite-svelte';
	import { DownloadSolid } from 'flowbite-svelte-icons';
	import { onMount } from 'svelte';
	import { scale, slide } from 'svelte/transition';

	export let lang: string = 'en';
	let downloadedLangs: string[] = [];
	let downloading: { lang: string; url: string } | undefined = undefined;

	function downloadHandlerFactory(song: OpenLpSongLang) {
		return async function () {
			if (!downloadedLangs.includes(song.url.split("/").pop()?.toLowerCase() ?? "__")) {
				downloading = { lang: song.language, url: song.url };
				try {
					await invoke('download_openlp_song', { url: song.url });
					downloading = undefined;
					alerts.add({
						message: `Success: OpenLp ${song.language} Songs Database downloaded`,
						kind: 'success'
					});
				} catch (err) {
					downloading = undefined;
					alerts.add({ message: `Error: ${JSON.stringify(err)}`, kind: 'error' });
				}
			} else {
				lang = song.language;
			}
		};
	}

  onMount(async () => {
    let dbs = await invoke("get_downloaded_song_dbs", { format: "OpenLp" })
    downloadedLangs = dbs as string[];
  })
</script>

<Button color="alternative" size="xs">{lang.toUpperCase()}</Button>
<Dropdown transition={slide}>
	{#each OPENLP_SONGS as song}
		<DropdownItem
			class="flex justify-between w-full items-center gap-2"
			on:click={downloadHandlerFactory(song)}
		>
			<div>
				<span>{song.language}</span>
				<span>- {song.songCount} songs</span>
			</div>
			{#if !downloadedLangs.includes(song.url.split("/").pop()?.toLowerCase() ?? "__")}
				<DownloadSolid />
			{/if}
		</DropdownItem>
	{/each}
</Dropdown>

<Modal transition={scale} open={downloading !== undefined}>
	<Heading tag="h4">OpenLp Song Database - {downloading?.lang}</Heading>
	<div class="w-full flex items-center gap-4">
		<span>Downloading {downloading?.url.split('/').pop()}</span>
		<Spinner />
	</div>
</Modal>
