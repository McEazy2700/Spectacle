<script lang="ts">
	import type { SongFormatType } from '$lib/models/song';
	import { OpenLpSongLangs, SongFormat, SongList } from '.';

	let format: SongFormatType = 'OpenLp';
	let lang: string = 'en';
</script>

<div class="h-full">
	<SongFormat bind:format />
	{#if format === 'OpenLp'}
		<OpenLpSongLangs bind:lang/>
	{/if}
	<SongList bind:format bind:lang/>
</div>
