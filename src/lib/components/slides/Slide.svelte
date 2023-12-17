<script lang="ts">
	import { twMerge } from 'tailwind-merge';

	let klass = '';
	export { klass as class };
	export let position: 'center' | 'start' | 'end' = 'center';
	export let size: 'xs' | 'sm' | 'md' | 'lg' | 'full' = 'full';
	export let bordered = false;
	export let scalable = false;
	export let maxWidth: number | undefined = undefined;
	export let scale: number | undefined = undefined;
	export let focused = false;

	let slideElement: HTMLDivElement;

	$: {
		if (scale && slideElement && size !== 'full' && scalable) {
			slideElement.style.scale = scale.toString();
		}
	}
</script>

<div
	style={maxWidth ? `max-width: ${maxWidth}px; max-height: ${(maxWidth * 9) / 16}px` : ''}
	class={`aspect-video rounded-lg transition-all w-screen h-screen
		${size !== 'full' ? '' : ''}
    ${
			size === 'xs'
				? 'max-w-[300px] max-h-[169px] overflow-hidden'
				: size === 'sm'
				? 'max-w-sm'
				: ''
		}
    ${bordered ? 'border border-black/20 dark:border-white/20' : ''}
    ${focused ? 'border !border-primary-600 shadow-md shadow-primary-600/50' : ''}
  `}
>
	<div
		bind:this={slideElement}
		class={twMerge(
			`flex w-screen aspect-video origin-top-left
      ${position === 'center' ? ' items-center' : position === 'end' ? 'items-end' : 'items-start'}
      ${size === 'xs' ? ' scale-[0.24]' : ''}`,
			klass
		)}
	>
		<slot />
	</div>
</div>
