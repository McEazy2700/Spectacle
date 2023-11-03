<script lang="ts">
	import { P } from 'flowbite-svelte';
	import { slide } from 'svelte/transition';
</script>

<div
	transition:slide
	class="bg-black/10 rounded w-full flex items-center justify-center max-w-[200px] aspect-video"
>
	<P class="text-black/60 w-fit dark:text-white/50" size="lg" weight="bold">No Slides</P>
</div>
