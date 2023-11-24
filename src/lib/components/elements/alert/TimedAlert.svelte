<script lang="ts">
	import alerts, { type AlertData } from '$lib/stores/alerts';
	import { Alert } from 'flowbite-svelte';
	import { InfoCircleSolid } from 'flowbite-svelte-icons';
	import { onMount } from 'svelte';
	import { fade } from 'svelte/transition';

	export let alert: AlertData;
	export let index: number;

	onMount(() => {
		setTimeout(() => {
			alerts.remove(index);
		}, 7000);
	});
</script>

<div transition:fade>
	<Alert on:close={() => alerts.remove(index)}
		transition={fade}
		border
		dismissable
		color={alert.kind === 'success' ? 'green' : alert.kind === 'warning' ? 'yellow' : 'red'}
	>
		<InfoCircleSolid slot="icon" class="w-4 h-4" />
		{alert.message}
	</Alert>
</div>
