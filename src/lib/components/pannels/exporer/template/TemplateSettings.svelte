<script lang="ts">
	import { Input, Label } from 'flowbite-svelte';
	import { Alignment, FontStyle, FontWeightInput, MediaInput } from '.';
	import type { AlignmentType, FontStyleType } from '$lib/types';

	export let name: string;
	export let fontWeight = 400;
	export let fontSize = 30;
	export let fontColor = '';
	export let fontStyle: FontStyleType = 'normal';
	export let horizontalAlignment: AlignmentType = 'start';
	export let verticalAlignment: AlignmentType = 'start';
	export let textAlignment: AlignmentType = 'start';
	export let background: string | undefined = undefined;
	export let noBackground = false;
</script>

<div class="my-4 py-2 border-b border-black/20 dark:border-white/20">
	<Label class="text-lg font-semibold">{name}</Label>
</div>
<div class="grid gap-4 grid-cols-4 mt-3">
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
	{#if !noBackground}
		<div class="flex flex-col gap-1">
			<Label class="font-bold opacity-70">Background</Label>
			<MediaInput bind:src={background} />
		</div>
	{/if}
</div>
