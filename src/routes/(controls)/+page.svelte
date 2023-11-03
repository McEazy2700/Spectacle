<script lang="ts">
	import { PannelBar, Toolbar } from '$lib/components/utils';
	import { ActivePannel, SelectPannels } from '$lib/components/pannels';
	import { LiveView } from '$lib/components/views';
</script>

<div class="flex flex-col min-h-screen">
	<Toolbar />
	<div class="mt-7 flex-1 flex w-full gap-2">
		<ActivePannel />
		<div
			class="max-w-xs border shadow-md aspect-video border-black/10 dark:border-white/10 rounded"
		>
			<LiveView class="scale-[0.3] w-[1080px] origin-top-left" />
		</div>
	</div>
	<PannelBar />
	<div class="mt-3">
		<SelectPannels />
	</div>
</div>
