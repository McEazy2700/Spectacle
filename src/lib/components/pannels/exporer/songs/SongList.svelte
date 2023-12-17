<script lang="ts">
	import type { SongFormatType, SongModel, SongOptions } from '$lib/models/song';
	import activePreview from '$lib/stores/active/preview';
	import activeSong from '$lib/stores/active/song';
	import alerts from '$lib/stores/alerts';
	import editing from '$lib/stores/editing';
	import openPannels from '$lib/stores/pannels/openPannels';
	import { parseSongXML } from '$lib/utils/song';
	import { invoke } from '@tauri-apps/api';
	import { Input, Label } from 'flowbite-svelte';
	import { onMount } from 'svelte';

	export let format: SongFormatType;
	export let lang: string;
	let songs: SongModel[] = [];
	let search_text: string | undefined = undefined;
	let search_title: string | undefined = undefined;

	async function get_songs(
		format: SongFormatType,
		lang: string,
		offset?: number,
		limit?: number,
		search_title?: string,
		search_text?: string
	) {
		let opts: SongOptions = {
			lang: lang.toLowerCase(),
			limit,
			format,
			offset,
			search_text,
			search_title
		};
		try {
			const res = await invoke('get_songs', { opts: opts });
			songs = res as SongModel[];
		} catch (err) {
			alerts.add({ message: `Error: ${JSON.stringify(err)}`, kind: 'error' });
		}
	}

	onMount(async () => {
		get_songs(format, lang, undefined, 20, search_title, search_text);
	});
	$: get_songs(format, lang, undefined, 30, search_title, search_text);

  const activeSongHandlerFactory = (song: SongModel) => {
    return () => {
      if (!$openPannels.includes("Preview")) {
        openPannels.add("Preview")
      }
      activeSong.set({ song, lang, format })
      activePreview.set("Song")
      editing.set(false)
    }
  }
</script>

<ul class="h-[70%] mt-2 overflow-auto">
	<div class="flex gap-2">
		<div class="flex-[0.05] flex flex-col gap-2">
			<Label
				class={`w-full font-semibold bg-black/10 dark:bg-white/20 p-1
        rounded flex items-center justify-center`}
			>
				Id
			</Label>
			<Input disabled type="text" size="sm" class="rounded" placeholder="Id" />
		</div>
		<div class="flex-[0.2] flex flex-col gap-2">
			<Label class="bg-black/10 font-semibold dark:bg-white/20 p-1 px-3 rounded">Title</Label>
			<Input
				type="text"
				size="sm"
				class="rounded"
				bind:value={search_title}
				placeholder="Search Title"
			/>
		</div>
		<div class="flex-1 flex flex-col gap-2">
			<Label class="bg-black/10 font-semibold dark:bg-white/20 p-1 px-3 rounded">Lyrics</Label>
			<Input
				type="text"
				size="sm"
				class="rounded"
				bind:value={search_text}
				placeholder="Search Lyrics"
			/>
		</div>
	</div>
	{#each songs as song}
		<div
			on:keydown={activeSongHandlerFactory(song)}
			tabindex="0"
			on:click={activeSongHandlerFactory(song)}
			role="button"
			class="flex mt-1 group gap-2 items-center"
		>
			<span
				class={`p-1 group-hover:bg-primary-600 transition-all border-black/10
         dark:border-white/10 rounded flex-[0.04] w-full flex border
         h-full items-center justify-center`}
			>
				{song.id}
			</span>
			<span
				class={`flex-[0.185] p-1 group-hover:bg-primary-600 transition-all px-2
         rounded border-black/10 dark:border-white/10 border h-full`}
			>
				<span class="line-clamp-1">
					{song.title}
				</span>
			</span>
			<span
				title={parseSongXML(song.lyrics, song.title).verses[0].text}
				class={`flex-1 p-1 group-hover:bg-primary-600 transition-all px-2 rounded
         border border-black/10 dark:border-white/10 text-start`}
			>
				<span class="line-clamp-1">
					{parseSongXML(song.lyrics, song.title).verses[0].text}
				</span>
			</span>
		</div>
	{/each}
</ul>
