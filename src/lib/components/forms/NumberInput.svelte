<script lang="ts">
	import { ButtonGroup } from 'flowbite-svelte';
	import { twMerge } from 'tailwind-merge';
	import AngleDown from '~icons/uis/angle-down';
	import AngleUp from '~icons/uis/angle-up';

  let klass = ""
  export { klass as class }
	export let placeholder = '0';
	export let value = 0;
	export let size: 'sm' | 'md' | 'lg' = 'sm';

	const decrease = () => (value -= 1);
	const increase = () => (value += 1);
</script>

<ButtonGroup
	{size}
	class={twMerge("p-0 flex items-center border border-black/10 dark:border-white/10 rounded", klass)}
>
	<input bind:value {placeholder} class="bg-transparent text-xs px-2 w-10" />
	<div class="flex flex-col">
		<button
			on:click={increase}
			type="button"
			class={`border-l hover:bg-black/10 dark:hover:bg-white/10 border-b-0
        border-black/10 px-1 dark:border-white/10 rounded-tr-sm transition-all`}
		>
			<AngleUp font-size={10} />
		</button>
		<button
			on:click={decrease}
			type="button"
			class={`border-l hover:bg-black/10 dark:hover:bg-white/10 border-t
        border-black/10 px-1 dark:border-white/10 rounded-br-sm transition-all`}
		>
			<AngleDown font-size={10} />
		</button>
	</div>
</ButtonGroup>
