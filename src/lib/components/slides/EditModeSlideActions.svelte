<script lang="ts">
	import { IconButton } from '../elements';
	import AngleUp from '~icons/uis/angle-up';
	import AngleDown from '~icons/uis/angle-down';
	import { Popover } from 'flowbite-svelte';
	import { fade } from 'svelte/transition';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher<{ moveUp: null; moveDown: null }>();

	const handleMoveUp = () => dispatch('moveUp');
	const handleMoveDown = () => dispatch('moveDown');
</script>

<Popover placement="bottom-end" class="z-[1000]" transition={(e) => fade(e, { duration: 200 })}>
	<div class="flex flex-col gap-1">
		<IconButton
			on:click={handleMoveUp}
			class="p-1 flex items-center w-full justify-start gap-1 pr-2"
		>
			<span class="border-r-2 border-white/40 pr-1">
				<AngleUp class="text-sm" />
			</span>
			<span class="text-sm pl-1">Move Up</span>
		</IconButton>
		<IconButton
			on:click={handleMoveDown}
			class="p-1 w-full justify-start flex items-center gap-1 pr-2"
		>
			<span class="border-r-2 border-white/40 pr-1">
				<AngleDown class="text-sm" />
			</span>
			<span class="text-sm pl-1">Move Down</span>
		</IconButton>
	</div>
</Popover>
