<script lang="ts">
	import { SermonViewPannel, SpectaclePannel } from '$lib/components/pannels';
	import type { Pannel } from '$lib/types';

	export let pannels: Pannel[] = [];
	let activePannel: Pannel = pannels[0];
</script>

<div class="flex flex-col flex-1 relative w-full">
	<div class="p-1 w-full flex-1">
		{#if activePannel === 'Sermon'}
			<SermonViewPannel />
		{:else}
			<SpectaclePannel />
		{/if}
	</div>
</div>
