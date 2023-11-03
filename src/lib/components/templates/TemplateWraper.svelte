<script lang="ts">
	import { convertFileSrc } from '$lib/utils/media';
	import ImageBackground from './backgrounds/ImageBackground.svelte';
	import VideoBackground from './backgrounds/VideoBackground.svelte';
	import type { TemplateType } from '$lib/types';

	export let template: TemplateType;
	let fw = template.font_weight;
	let className = {
		fontWeight:
			fw === 'Bold'
				? 'font-bold'
				: fw === 'SemiBold'
				? 'font-semibold'
				: fw === 'ExtraBold'
				? 'font-extrabold'
				: fw === 'Light'
				? 'font-light'
				: ''
	};
</script>

<div class="relative min-h-full min-w-full z-0">
	<div class="z-[-1000] absolute inset-0">
		{#if template.background_type === 'Video' && template.background_url}
			<VideoBackground src={convertFileSrc(template.background_url)} />
		{:else if template.background_type === 'Image' && template.background_url}
			<ImageBackground src={convertFileSrc(template.background_url)} />
		{/if}
	</div>
	<div
		class={`${className.fontWeight} w-full h-full z-50`}
		style={`font-size: ${template.font_size}px;`}
	>
		<slot />
	</div>
</div>
