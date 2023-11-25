<script lang="ts">
	import { Pannel, SermonViewPannel } from '$lib/components/pannels';
	import activePannel from '$lib/stores/pannels/activePannel';
	import { Slide } from '../slides';
</script>

<Pannel video class="!aspect-video">
	{#if $activePannel}
		<div class="p-1 flex-1">
			<SermonViewPannel />
		</div>
	{:else}
		<Slide size="lg" />
	{/if}
</Pannel>
