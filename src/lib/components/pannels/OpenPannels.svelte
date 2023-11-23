<script lang="ts">
	import openPannels from '$lib/stores/pannels/openPannels';
	import { ActivePannel, Explorer, Schedule } from '$lib/components/pannels';
	import { LiveView } from '../views';
</script>

<div class="min-w-full flex flex-col h-screen">
	<div class="flex mb-2 flex-1 h-full justify-between w-full gap-2 items-start">
		<div>
			{#if $openPannels.includes('Schedule')}
				<Schedule />
			{/if}
		</div>
		{#if $openPannels.includes('Preview')}
			<ActivePannel />
		{/if}
		{#if $openPannels.includes('LiveView')}
			<div class="justify-self-end w-fit">
				<LiveView size="xs" bordered />
			</div>
		{/if}
	</div>
	{#if $openPannels.includes('Explorer')}
    <div class="flex-1 h-full">
    <Explorer />
    </div>
  {/if}
</div>
