<script lang="ts">
	import { Spinner } from 'flowbite-svelte';
	import { SpectacleLogo } from '..';
</script>

<div class="flex items-center justify-center gap-2">
	<SpectacleLogo class="inline w-fit px-3" />
	<Spinner class="inline" color="gray" />
</div>
