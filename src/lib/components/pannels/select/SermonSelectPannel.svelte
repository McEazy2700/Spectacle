<script>
	import { IconButton } from '$lib/components/elements';
	import activePannel from '$lib/stores/pannels/activePannel';

	function handleAdd() {
		activePannel.set({ type: 'Sermon' });
	}
</script>

<div class="w-full flex flex-col p-2 relative">
	<IconButton class="self-center" on:click={handleAdd} variant="add" />
</div>
