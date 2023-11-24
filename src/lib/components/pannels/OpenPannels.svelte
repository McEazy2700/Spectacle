<script lang="ts">
	import openPannels from '$lib/stores/pannels/openPannels';
	import { ActivePannel, Explorer, Resize, Schedule } from '$lib/components/pannels';
	import { LiveView } from '../views';
</script>

<div class="min-w-full flex flex-col h-screen">
	<div id="work-pannels" class="flex p-3 mb-2 min-h-[20%] justify-between w-full gap-2 items-start">
		{#if $openPannels.includes('Schedule')}
			<Schedule />
		{/if}
		{#if $openPannels.includes('Preview')}
			<ActivePannel />
		{/if}
		{#if $openPannels.includes('LiveView')}
			<div class="justify-self-end w-fit">
				<LiveView size="xs" bordered />
			</div>
		{/if}
	</div>
	<Resize itemId="work-pannels" direction="vertical" />
	{#if $openPannels.includes('Explorer')}
		<Explorer />
	{/if}
</div>
