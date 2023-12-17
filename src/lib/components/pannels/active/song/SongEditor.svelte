<script lang="ts">
	import ScreenRounded from '~icons/material-symbols/fit-screen-rounded';
	import { Button, Input, Spinner, Textarea } from 'flowbite-svelte';
	import { VerseTypeDropdown } from '.';
	import { songEditing } from '$lib/stores/active/song';
	import type { SongData, SongModel, SongUpdateOptions } from '$lib/models/song';
	import editing from '$lib/stores/editing';
	import { stringifySongToXML } from '$lib/utils/song';
	import { invoke } from '@tauri-apps/api';
	import alerts from '$lib/stores/alerts';

	export let containerHeight: number;
	export let song: SongData;
	export let title: string | undefined = undefined;
	export let currentVerseText: string;
	export let currentVerseType: 'c' | 'v' | undefined = undefined;
	export let previewOpen = false;
	let loading = false;

	const updateSong = () => {
		song.verses[$songEditing?.activeVerseIndex ?? 0].text = currentVerseText;
		song = song;
	};

	const updateVerseType = (e: CustomEvent<'c' | 'v'>) => {
		if ($songEditing && currentVerseType) {
			song.verses[$songEditing.activeVerseIndex].type = e.detail;
			song = song;
		}
	};

	const handleSave = async () => {
		if ($songEditing?.song?.id) {
			const songStr = stringifySongToXML(song);
			const updateOpts: SongUpdateOptions = {
				title: title,
				id: $songEditing.song.id,
				lang: $songEditing.lang ?? 'en',
				format: $songEditing.format ?? 'OpenLp',
				lyrics: songStr
			};
			try {
				const res: SongModel = await invoke('update_song', { opts: updateOpts });
				alerts.add({ kind: 'success', message: `Success: ${res.title} updated` });
			} catch (err) {
				alerts.add({ kind: 'error', message: `Error: ${err}` });
			}
		}
	};
</script>

<div style={`height: ${containerHeight}px;`} class="flex-1 p-1 flex flex-col gap-2 overflow-y-auto">
	<div class="flex items-center justify-between">
		<VerseTypeDropdown on:change={updateVerseType} bind:type={currentVerseType} />
		<Button
			on:click={() => (previewOpen = !previewOpen)}
			type="button"
			title="Preview"
			size="xs"
			color="alternative"
			class="p-2 px-4"
		>
			<ScreenRounded font-size={15} />
		</Button>
	</div>
	<Input
		bind:value={title}
		type="text"
		placeholder="Song Title"
		size="md"
		class="rounded-md text-lg font-bold"
	/>
	<Textarea
		on:keyup={updateSong}
		class="border-white/40 min-h-[250px]"
		bind:value={currentVerseText}
		rows={15}
		placeholder="Type Something"
	/>
	<div class="self-end mt-auto">
		<Button
			on:click={() => editing.set(false)}
			title="Close Edit"
			color="alternative"
			class="rounded-md"
			type="button"
			size="xs"
		>
			Close
		</Button>
		<Button on:click={handleSave} title="Save Edit" class="rounded-md" type="button" size="xs">
			{#if loading}
				<Spinner color="white" />
			{:else}
				Save
			{/if}
		</Button>
	</div>
</div>
