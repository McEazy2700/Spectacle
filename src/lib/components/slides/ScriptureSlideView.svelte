<script lang="ts">
	import type { ScriptureSlide } from '$lib/models/slide';
	import type { TemplateModel } from '$lib/models/template';
	import alerts from '$lib/stores/alerts';
	import { invoke } from '@tauri-apps/api';
	import { Background } from '../templates';

	export let slide: ScriptureSlide;
	let template: TemplateModel | undefined;

	const getTemplates = async () => {
		try {
			template = await invoke('get_template', { id: slide.Scripture.template_id });
		} catch (err) {
			alerts.add({ message: `Error: ${JSON.stringify(err)}`, kind: 'error' });
		}
	};
	$: getTemplates();
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

<Background src={template?.background} class="w-[100vw] overflow-hidden h-[100vh] p-10">
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
		<p>
			{slide.Scripture.text}
		</p>
		<p
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
			{slide.Scripture.passage}
		</p>
	</div>
</Background>

<style>
	p {
		text-shadow: var(--text-shadow);
	}
	.side-text {
		text-shadow: var(--side-text-shadow);
	}
</style>
