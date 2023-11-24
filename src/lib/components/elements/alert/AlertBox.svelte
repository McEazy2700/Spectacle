<script lang="ts">
	import alerts from '$lib/stores/alerts';
	import { fade } from 'svelte/transition';
	import { TimedAlert } from '.';
</script>

{#if $alerts.length > 0}
	<ul transition:fade class="fixed bottom-1 flex flex-col gap-3 right-2 z-[1000]">
		{#each $alerts as alert, id}
			<TimedAlert {alert} index={id} />
		{/each}
	</ul>
{/if}
