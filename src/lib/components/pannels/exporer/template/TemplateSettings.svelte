<script lang="ts">
	import { Input, Label } from 'flowbite-svelte';
	import { Alignment, BooleanDropdown, FontStyle, FontWeightInput, MediaInput } from '.';
	import type { AlignmentType, FontStyleType } from '$lib/types';

	export let name: string;
	export let fontWeight = 400;
	export let fontSize = 30;
	export let fontColor = '#ffffff';
	export let fontStyle: FontStyleType = 'normal';
	export let horizontalAlignment: AlignmentType = 'start';
	export let verticalAlignment: AlignmentType = 'start';
	export let textAlignment: AlignmentType = 'start';
  export let textShadow: boolean = false;
  export let textShadowBlur: number = 1;
  export let textShadowColor: string = "#E2510C"
  export let textShadowVertical: number = 2;
  export let textShadowHorizontal: number = 2;
	export let background: string | undefined = undefined;
	export let noBackground = false;
</script>

<div class="my-4 py-2 border-b border-black/20 dark:border-white/20">
	<Label class="text-lg font-semibold">{name}</Label>
</div>
<div class="grid w-full gap-4 grid-cols-4 mt-3">
	<div class="flex flex-col gap-1">
		<Label class="font-bold opacity-70">Font Weight</Label>
		<FontWeightInput bind:weight={fontWeight} />
	</div>
	<div class="flex flex-col gap-1">
		<Label class="font-bold opacity-70">Font Style</Label>
		<FontStyle bind:fontStyle />
	</div>
	<div class="flex flex-col gap-1">
		<Label class="font-bold opacity-70">Font Size</Label>
		<Input
			class="max-w-[5rem] rounded"
			bind:value={fontSize}
			size="sm"
			type="number"
			placeholder={30}
		/>
	</div>
	<div class="flex flex-col gap-1">
		<Label class="font-bold opacity-70">Text Color</Label>
		<input
			class="bg-transparent w-full h-full max-w-[5rem] rounded"
			bind:value={fontColor}
			type="color"
		/>
	</div>
	<div class="flex flex-col gap-1">
		<Label class="font-bold opacity-70">Horizontal Align</Label>
		<Alignment bind:alignment={horizontalAlignment} />
	</div>
	<div class="flex flex-col gap-1">
		<Label class="font-bold opacity-70">Vertical Align</Label>
		<Alignment bind:alignment={verticalAlignment} />
	</div>
	<div class="flex flex-col gap-1">
		<Label class="font-bold opacity-70">Text Align</Label>
		<Alignment bind:alignment={textAlignment} />
	</div>
	<div class="flex flex-col gap-1">
		<Label class="font-bold opacity-70">Text Shadow</Label>
    <BooleanDropdown bind:value={textShadow}/>
	</div>
	<div class="flex flex-col gap-1">
		<Label class="font-bold opacity-70">Text Shadow Vertical</Label>
		<Input
			class="max-w-[5rem] rounded"
			bind:value={textShadowVertical}
			size="sm"
			type="number"
			placeholder={30}
		/>
	</div>
	<div class="flex flex-col gap-1">
		<Label class="font-bold opacity-70">Text Shadow Horizontal</Label>
		<Input
			class="max-w-[5rem] rounded"
			bind:value={textShadowHorizontal}
			size="sm"
			type="number"
			placeholder={30}
		/>
	</div>
	<div class="flex flex-col gap-1">
		<Label class="font-bold opacity-70">Text Shadow Blur</Label>
		<Input
			class="max-w-[5rem] rounded"
			bind:value={textShadowBlur}
			size="sm"
			type="number"
			placeholder={30}
		/>
	</div>
	<div class="flex flex-col gap-1">
		<Label class="font-bold opacity-70">Text Shadow Color</Label>
		<input
			class="bg-transparent w-full h-full max-w-[5rem] rounded"
			bind:value={textShadowColor}
			type="color"
		/>
	</div>
	{#if !noBackground}
		<div class="flex flex-col gap-1">
			<Label class="font-bold opacity-70">Background</Label>
			<MediaInput bind:src={background} />
		</div>
	{/if}
</div>
