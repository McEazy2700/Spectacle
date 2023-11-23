<script lang="ts">
	import { Pannel, SermonViewPannel, TemplatePannel } from '$lib/components/pannels';
	import activePannel from '$lib/stores/pannels/activePannel';
	import { Slide } from '../slides';
</script>

<Pannel>
	{#if $activePannel}
		<div class="p-1 w-full aspect-video flex-1">
				<!-- <SermonViewPannel /> -->
				<!-- <TemplatePannel /> -->
		</div>
	{:else}
		<Slide size="lg" />
	{/if}
</Pannel>
