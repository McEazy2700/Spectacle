<script lang="ts">
	import { Background } from '$lib/components/templates';
	import { imageExtensions } from '$lib/constants/extensions';
	import type { AlignmentType, FontStyleType } from '$lib/types';
	import { convertFileSrc } from '$lib/utils/media';

	export let fontSize: number;
	export let fontColor: string | undefined;
	export let fontStyle: FontStyleType;
	export let fontWeight: number;
	export let textShadow: boolean;
	export let textShadowBlur: number;
	export let textShadowColor: string;
	export let textShadowVertical: number;
	export let textShadowHorizontal: number;
	export let textAlignment: AlignmentType;
	export let verticalAlignment: AlignmentType;
	export let horizontalAlignment: AlignmentType;
	export let background: string | undefined;

	export let sideTextFontSize: number;
	export let sideTextFontColor: string | undefined;
	export let sideTextFontStyle: FontStyleType;
	export let sideTextFontWeight: number;
	export let sideTextShadow: boolean;
	export let sideTextShadowBlur: number;
	export let sideTextShadowColor: string;
	export let sideTextShadowVertical: number;
	export let sideTextShadowHorizontal: number;
	export let sideTextHorizontalAlignment: AlignmentType;
	export let sideTextVerticalAlignment: AlignmentType;
	export let sideTextTextAlignment: AlignmentType;
	export let size: 'xs' | 'base' = 'base';

	function makeShadow(hor: number, ver: number, blur: number, col: string): string {
		return `${hor}px ${ver}px ${blur}px ${col}`;
	}

	$: textShadowStyle = `--text-shadow: ${
		textShadow
			? makeShadow(textShadowHorizontal, textShadowVertical, textShadowBlur, textShadowColor)
			: 'none'
	};`;

	$: sideTextShadowStyle = `--side-text-shadow: ${
		sideTextShadow
			? makeShadow(
					sideTextShadowHorizontal,
					sideTextShadowVertical,
					sideTextShadowBlur,
					sideTextShadowColor
			  )
			: 'none'
	};`;
</script>

<Background
	class={`${size === 'xs' ? 'p-0' : 'mt-3 p-3'} min-w-full py-0 aspect-video rounded-lg`}
	src={background && size === 'xs' && !imageExtensions.includes(background.split('.').pop() ?? '__')
		? convertFileSrc(background, { thumbnail: true })
		: background}
	bordered
>
	<div
		contenteditable
		style={`
        ${textShadowStyle}
        ${sideTextShadowStyle}
          font-weight: ${fontWeight};
          font-size: ${fontSize}px;
          align-items: ${horizontalAlignment};
          justify-content: ${verticalAlignment};
          color: ${fontColor};
          font-style: ${fontStyle};
          text-align: ${textAlignment}
        `}
		class={`rounded-lg flex flex-col relative z-0 ${
			size === 'xs'
				? 'scale-[0.43] w-[720px] h-[405px] aspect-video origin-top-left'
				: 'w-full h-full '
		}`}
	>
		<div>Main Text</div>
		<div
			class={`absolute z-10 side-text ${
				sideTextHorizontalAlignment === 'center'
					? 'left-[42%]'
					: sideTextHorizontalAlignment === 'end'
					? 'right-4'
					: 'left-3'
			} ${
				sideTextVerticalAlignment === 'center'
					? 'top-[46%]'
					: sideTextVerticalAlignment === 'end'
					? 'bottom-3'
					: 'top-3'
			}`}
			style={`
          font-weight: ${sideTextFontWeight};
          font-size: ${sideTextFontSize}px;
          color: ${sideTextFontColor};
          font-style: ${sideTextFontStyle};
          text-align: ${sideTextTextAlignment}
        `}
		>
			Side Text
		</div>
	</div>
</Background>

<style>
	div {
		text-shadow: var(--text-shadow);
	}
  .side-text {
    text-shadow: var(--side-text-shadow);
  }
</style>
