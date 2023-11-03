<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import { StandbyView, View as ViewComponent, SermonViewDisplay } from '.';
	import { TemplateWraper } from '$lib/components/templates';
	import { Template, type SermonView, type View } from '$lib/classes/views';

	let template: Template = new Template();
	let sermonView: SermonView | undefined = undefined;
	let klass = '';
	export { klass as class };

	listen('presentation-update', (e) => {
		const view = (e.payload as { view: View }).view;
		if (view.name === 'Sermon') {
			sermonView = view.data as SermonView;
		}
	});
</script>

<TemplateWraper bind:template>
	<ViewComponent bind:class={klass}>
		{#if sermonView}
			<SermonViewDisplay bind:sermonView />
		{:else}
			<StandbyView />
		{/if}
	</ViewComponent>
</TemplateWraper>
