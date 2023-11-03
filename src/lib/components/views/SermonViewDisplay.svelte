<script lang="ts">
	import type { SermonView } from '$lib/classes/views';

	export let sermonView: SermonView;
</script>

<div class="w-full h-full">
	<!-- eslint-disable-next-line svelte/no-at-html-tags -->
	{@html sermonView.content}
</div>
