<script>
	import { IconButton } from '$lib/components/elements';
	// import activePannel from '$lib/stores/pannels/activePannel';
	import { PlusSolid } from 'flowbite-svelte-icons';

	function handleAdd() {
	}
</script>

<div class="w-full flex flex-col p-2 relative">
	<IconButton class="self-center" on:click={handleAdd}>
    <PlusSolid />
	</IconButton>
</div>
