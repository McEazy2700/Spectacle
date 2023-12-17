<script lang="ts">
	import type { SongFormatType } from '$lib/models/song';
	import { Button, Dropdown, DropdownItem } from 'flowbite-svelte';
	import { slide } from 'svelte/transition';

	export let format: SongFormatType = 'OpenLp';
</script>

<Button color="alternative" size="xs">{format}</Button>
<Dropdown transition={slide}>
	<DropdownItem>OpenLp</DropdownItem>
</Dropdown>
