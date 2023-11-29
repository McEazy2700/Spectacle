<script lang="ts">
	import { DesktopPcSolid, FolderDuplicateSolid, RectangleListSolid, VideoSolid } from 'flowbite-svelte-icons';
	import openPannels from '$lib/stores/pannels/openPannels';
  import type { Pannel } from "$lib/models/pannels"
	import { IconButton } from '../elements';

  function pannelToggleFactory(pannel: Pannel) {
    return function() {
      if ($openPannels.includes(pannel)) {
        openPannels.remove(pannel)
      } else {
        openPannels.add(pannel)
      }
    }
  }
</script>

<div class="top-1/4 absolute group z-[1000]">
	<div
		class="bg-primary-600 h-16 w-2 group-hover:w-8 group-hover:h-20 transition-all rounded-r-lg"
	/>
	<div class="relative">
		<div
			class={`
      bg-primary-700 p-1 absolute group-hover:left-0 -top-16 transition-all
      -left-56 py-2 rounded-r-xl flex flex-col gap-2
    `}
		>
			<IconButton
				active={$openPannels.includes('Schedule')}
				title="Schedule"
				circle
				on:click={pannelToggleFactory("Schedule")}
			>
				<RectangleListSolid />
			</IconButton>
			<IconButton
				on:click={pannelToggleFactory("LiveView")}
				active={$openPannels.includes('LiveView')}
				circle
				title="Live"
			>
				<VideoSolid />
			</IconButton>
			<IconButton
				on:click={pannelToggleFactory("Preview")}
				active={$openPannels.includes('Preview')}
				circle
				title="Preview"
			>
        <DesktopPcSolid />
			</IconButton>
			<IconButton
				on:click={pannelToggleFactory("Explorer")}
				active={$openPannels.includes('Explorer')}
				circle
				title="Explorer"
			>
        <FolderDuplicateSolid />
			</IconButton>
		</div>
	</div>
</div>
