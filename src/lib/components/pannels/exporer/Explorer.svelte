<script>
	import { TabItem, Tabs } from 'flowbite-svelte';
	import { FileExplorer, Pannel } from '..';
	import { CaretRightSolid, ListMusicSolid, OpenBookSolid, PenSolid } from 'flowbite-svelte-icons';
	import { ScriptureExplorer } from './scripture';
</script>

<Pannel class="min-h-[50vh] h-full">
	<Tabs style="underline">
		<TabItem class="h-full" open>
			<div class="flex items-center gap-2" slot="title">
				<CaretRightSolid />
				Media
			</div>
			<FileExplorer />
		</TabItem>
		<TabItem class="h-full">
			<div class="flex items-center gap-2" slot="title">
				<OpenBookSolid />
				Scripture
			</div>
			<ScriptureExplorer />
		</TabItem>
		<TabItem class="h-full">
			<div class="flex items-center gap-2" slot="title">
				<PenSolid />
				Sermon
			</div>
			Sermon
		</TabItem>
		<TabItem class="h-full">
			<div class="flex items-center gap-2" slot="title">
				<ListMusicSolid />
				Song
			</div>
			Song
		</TabItem>
	</Tabs>
</Pannel>
