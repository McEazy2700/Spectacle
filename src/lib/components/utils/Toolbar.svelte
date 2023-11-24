<script>
	import { Button, DarkMode, Toolbar } from 'flowbite-svelte';
	import { slide } from 'svelte/transition';
	import { GoLive } from '.';
	import { AngleDownSolid, AngleUpSolid } from 'flowbite-svelte-icons';

	let show = true;
</script>

<div class="relative shadow rounded-md z-[1000]">
	{#if show}
		<Toolbar class="z-20" transition={slide} embedded={false} color="dark">
			<div class="flex items-center justify-between w-screen max-w-[98vw]">
				<div>
					<DarkMode size="sm" />
				</div>
				<GoLive />
			</div>
		</Toolbar>
	{/if}
	<Button
		type="button"
		color="alternative"
		on:click={() => (show = !show)}
		size="xs"
		class={`
      absolute border-none shadow bg-gray-50 hover:bg-gray-100 dark:hover:bg-gray-700
      transition-all text-black dark:text-white dark:bg-gray-800 -z-10 right-5 -bottom-6
    `}
	>
		{#if show}
			<AngleUpSolid class="w-3 h-3" />
		{:else}
			<AngleDownSolid class="w-3 h-3" />
		{/if}
	</Button>
</div>
