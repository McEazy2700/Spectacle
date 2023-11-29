<script lang="ts">
	import type { ScriptureSlide, SlideType, TemplateView } from '$lib/models/slide';
	import { draggable } from '$lib/actions/dnd';
	import { ScriptureSlideView, SpectacleSlide } from '.';

	export let slide: SlideType | undefined = undefined;
	export let size: 'xs' | 'sm' | 'md' | 'lg' | 'full' = 'full';
	export let bordered = false;
	export let templateType: TemplateView | undefined = undefined;
  export let scale: number | undefined = undefined;

  let scriptureSlide: ScriptureSlide | undefined = undefined;

	let slideElement: HTMLDivElement;
	let containerElement: HTMLDivElement;

  $: {
    if (templateType === "Scripture") {
      scriptureSlide = slide as ScriptureSlide
    }
  }

  $: {
    if (scale && slideElement && size !== 'full' && size !== "xs") {
			slideElement.style.scale = scale.toString();
    }
  }
</script>

<div
	class={`aspect-video rounded-lg transition-all w-screen h-full
		${size !== 'full' ? '' : ''}
    ${size === 'xs' ? 'max-w-xs max-h-fit overflow-hidden' : size === 'sm' ? 'max-w-sm' : ''}
    ${bordered ? 'border border-black/20 dark:border-white/20' : ''}
  `}
	bind:this={containerElement}
>
	<div
		bind:this={slideElement}
		class="flex w-screen aspect-video items-center origin-top-left {size === 'xs' ? 'scale-[0.24]' : ''}"
	>
		{#if !slide}
			<SpectacleSlide />
		{:else if scriptureSlide}
			<ScriptureSlideView slide={scriptureSlide} />
		{:else}
			<div use:draggable={slide}>
				<p>Some</p>
			</div>
		{/if}
	</div>
</div>
