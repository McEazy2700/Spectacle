<script lang="ts">
	import type { TemplateModel } from '$lib/models/template';
	import { Background } from './backgrounds';

	export let template: TemplateModel | undefined;
  export let preview = false

	function makeShadow(hor: number, ver: number, blur: number, col: string): string {
		return `${hor}px ${ver}px ${blur}px ${col}`;
	}

	$: textShadowStyle = `--text-shadow: ${
		template?.text_shadow
			? makeShadow(
					template.text_shadow_horizontal,
					template.text_shadow_vertical,
					template.text_shadow_blur,
					template.text_shadow_color
			  )
			: 'none'
	};`;

	$: sideTextShadowStyle = `--side-text-shadow: ${
		template?.side_text_shadow
			? makeShadow(
					template.side_text_shadow_horizontal,
					template.side_text_shadow_vertical,
					template.side_text_shadow_blur,
					template.side_text_shadow_color
			  )
			: 'none'
	};`;
</script>

<Background {preview} src={template?.background} class="w-[100vw] overflow-hidden h-[100vh] p-10">
	<div
		class="flex flex-col w-full h-full relative z-0"
		style={`${textShadowStyle} ${sideTextShadowStyle}
          font-weight: ${template?.font_weight};
          font-size: ${template?.font_size}px;
          align-items: ${template?.horizontal_alignment};
          justify-content: ${template?.vertical_alignment};
          color: ${template?.font_color};
          font-style: ${template?.font_style};
          text-align: ${template?.text_alignment}
	`}
	>
		<div class="text whitespace-break-spaces">
			<slot name="main" />
		</div>
		<div
			class="side-text"
			style={`
					${
						template?.side_text_vertical_alignment === 'center'
							? 'top: 46%;'
							: template?.side_text_vertical_alignment === 'end'
							? 'bottom: 0.75rem;'
							: 'top: 0.75rem;'
					}
					${
						template?.side_text_horizontal_alignment === 'center'
							? 'left: 42%;'
							: template?.side_text_horizontal_alignment === 'end'
							? 'right: 1rem;'
							: 'left: 0.75rem;'
					}
					width: fit-content;
					z-index: 10;
					position: absolute;
          font-weight: ${template?.side_text_font_weight};
          font-size: ${template?.side_text_font_size}px;
          color: ${template?.side_text_font_color};
          font-style: ${template?.side_text_font_style};
          text-align: ${template?.side_text_text_alignment}
        `}
		>
			<slot name="side" />
		</div>
	</div>
</Background>

<style>
	.text {
		text-shadow: var(--text-shadow);
	}
	.side-text {
		text-shadow: var(--side-text-shadow);
	}
</style>
