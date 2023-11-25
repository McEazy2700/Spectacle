<script lang="ts">
	import { Background } from '$lib/components/templates';
	import type { AlignmentType, FontStyleType } from '$lib/types';

	export let fontWeight: number;
	export let fontSize: number;
	export let fontColor: string | undefined;
	export let fontStyle: FontStyleType;
	export let horizontalAlignment: AlignmentType;
	export let verticalAlignment: AlignmentType;
	export let textAlignment: AlignmentType;
	export let background: string | undefined;

	export let sideTextFontWeight: number;
	export let sideTextFontSize: number;
	export let sideTextFontColor: string | undefined;
	export let sideTextFontStyle: FontStyleType;
	export let sideTextHorizontalAlignment: AlignmentType;
	export let sideTextVerticalAlignment: AlignmentType;
	export let sideTextTextAlignment: AlignmentType;
	export let size: 'xs' | 'base' = 'base';
</script>

<Background class={`${size === "xs" ? "p-0" : "p-3"} py-0 aspect-video mt-3 rounded-lg`} src={background} bordered>
	<div
		contenteditable
		style={`
          font-weight: ${fontWeight};
          font-size: ${fontSize}px;
          align-items: ${horizontalAlignment};
          justify-content: ${verticalAlignment};
          color: ${fontColor};
          font-style: ${fontStyle};
          text-align: ${textAlignment}
        `}
		class={`rounded-lg flex flex-col w-full h-full relative z-0 ${
			size === 'xs' ? 'scale-[0.29] w-[1080px] h-[607px] aspect-video origin-top-left' : ''
		}`}
	>
		<div>Main Text</div>
		<div
			class={`absolute z-10 backdrop-blur-sm ${
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
