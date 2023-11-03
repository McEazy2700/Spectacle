<script lang="ts">
	import { Template } from '$lib/classes/views';
	import { TemplateWraper } from '$lib/components/templates';
	import type { FontWeight } from '$lib/types';
	import { TemplateSettings } from '.';

	let templateId: number | undefined = undefined;
	let templateName: string | undefined = undefined;
	let font: string | undefined = undefined;
	let fontSize: number | undefined = undefined;
	let fontWeight: FontWeight | undefined = undefined;
	let backgroundType: 'Video' | 'Image' = 'Image';
	let backgroundURL: string | undefined = undefined;

	$: template = new Template({ font, fontSize, fontWeight, backgroundType, backgroundURL });
</script>

<div class="border gap-3 p-2 flex w-full border-black/10 dark:border-white/10 rounded">
	<div class="flex-1 w-full aspect-video">
		<TemplateWraper bind:template>
			<div>
				<p
					style={`font-size: ${fontSize}px;`}
					class={`${
						fontWeight === 'Light'
							? 'font-light'
							: fontWeight === 'SemiBold'
							? 'font-semibold'
							: fontWeight === 'Bold'
							? 'font-bold'
							: fontWeight === 'ExtraBold'
							? 'font-extrabold'
							: ''
					}`}
				>
					Text
				</p>
			</div>
		</TemplateWraper>
	</div>
	<TemplateSettings
		bind:fontSize
		bind:fontWeight
		bind:templateName
		bind:templateId
		bind:backgroundURL
		bind:backgroundType
	/>
</div>
