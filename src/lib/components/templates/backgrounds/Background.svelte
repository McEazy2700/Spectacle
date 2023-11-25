<script lang="ts">
	import { imageExtensions } from '$lib/constants/extensions';
	import { twMerge } from 'tailwind-merge';
	import { ImageBackground, VideoBackground } from '.';

	let klass = '';
	export let src: string | undefined = undefined;
	export let bordered = false;
	export { klass as class };

	$: isImage = imageExtensions.includes(src?.split('.').pop() ?? '__');
</script>

<div
	class={twMerge(
		`z-0 relative overflow-hidden ${bordered ? 'border border-black/10 dark:border-white/20' : ''}`,
		klass
	)}
>
	{#if src}
		<div class="absolute w-full h-full inset-0 z-[-10]">
			{#if isImage}
				<ImageBackground {src} />
			{:else}
				<VideoBackground {src} />
			{/if}
		</div>
	{/if}
	<div class="z-10 w-full h-full">
		<slot />
	</div>
</div>
