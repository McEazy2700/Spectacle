<script lang="ts">
	import { SermonViewPannel, SpectaclePannel, TemplatePannel } from '$lib/components/pannels';
	import activePannel from '$lib/stores/pannels/activePannel';
</script>

<div class="flex flex-col flex-1 relative w-full">
	<div class="p-1 w-full flex-1">
		{#if $activePannel?.type === 'Sermon'}
			<SermonViewPannel />
		{:else if $activePannel?.type === 'Template'}
			<TemplatePannel />
		{:else}
			<SpectaclePannel />
		{/if}
	</div>
</div>
