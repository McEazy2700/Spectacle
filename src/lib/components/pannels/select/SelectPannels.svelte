<script lang="ts">
	import selectPannel from '$lib/stores/pannels/selectPannel';
	import { SermonSelectPannel, MediaSelectPannel, TemplateSelectPannel } from '.';
</script>

{#if $selectPannel === 'Sermon'}
	<SermonSelectPannel />
{:else if $selectPannel === 'Template'}
	<TemplateSelectPannel />
{:else if $selectPannel === 'Media'}
	<MediaSelectPannel />
{/if}
