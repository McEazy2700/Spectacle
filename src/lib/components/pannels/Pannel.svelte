<script lang="ts">
	import { fade } from 'svelte/transition';
	import { twMerge } from 'tailwind-merge';

	let klass = '';
	export { klass as class };
</script>

<div
	transition:fade
	class={twMerge('border-black/10 dark:border-white/10 h-full border rounded-xl w-full', klass)}
>
	<slot />
</div>
