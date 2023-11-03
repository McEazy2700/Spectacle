<script lang="ts">
	import Icon from '@iconify/svelte';
	import { Button } from 'flowbite-svelte';
	import { createEventDispatcher } from 'svelte';
	import { twMerge } from 'tailwind-merge';

	let klass = '';
	export { klass as class };
	export let title: string | undefined = undefined;
	export let variant: 'add' | 'edit' | 'back';
	export let disabled = false;
	const dispatch = createEventDispatcher<{ click: MouseEvent }>();

	function handleClick(e: MouseEvent) {
		dispatch('click', e);
	}
</script>

<Button
	bind:disabled
	on:click={handleClick}
	{title}
	class={twMerge(
		'w-fit self-end rounded bg-black/10 dark:bg-white/10 border-black/20 dark:border-white/20',
		klass
	)}
	color="alternative"
	size="xs"
>
	<Icon
		class="text-lg"
		icon={variant === 'add'
			? 'mingcute:add-fill'
			: variant === 'edit'
			? 'iconamoon:edit-fill'
			: 'eva:arrow-ios-back-outline'}
	/>
</Button>
