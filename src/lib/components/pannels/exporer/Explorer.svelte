<script>
	import { TabItem, Tabs } from 'flowbite-svelte';
	import { FileExplorer, Pannel, TemplateExplorer } from '..';
	import { CaretRightSolid, GridPlusSolid, ListMusicSolid, OpenBookSolid, PenSolid } from 'flowbite-svelte-icons';
	import { ScriptureExplorer, SongsExplorer } from '.';
</script>

<Pannel id="explore-pannels" class="flex-1 overflow-auto min-h-[30%] h-full">
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
			<SongsExplorer />
		</TabItem>
		<TabItem class="h-full">
			<div class="flex items-center gap-2" slot="title">
				<GridPlusSolid />
				Template
			</div>
			<TemplateExplorer />
		</TabItem>
	</Tabs>
</Pannel>
