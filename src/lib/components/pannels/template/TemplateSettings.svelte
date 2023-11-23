<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { Button, Dropdown, DropdownItem, Input, Modal, P, Spinner } from 'flowbite-svelte';
	import { scale, slide } from 'svelte/transition';
	import { convertFileSrc } from '$lib/utils/media';
	import type { FontWeight } from '$lib/types';
	import { videoExtensions } from '$lib/constants/extensions';

	export let templateId: number | undefined = undefined;
	export let templateName = '';
	export let fontSize = 16;
	export let fontWeight: FontWeight = 'Normal';
	export let fontWeightOpen = false;
	export let backgroundURL: string | undefined = undefined;
	export let backgroundType: 'Image' | 'Video' = 'Image';
	let bgPickerOpen = false;
	let loading = false;

	function weighFactory(weight: FontWeight) {
		return function () {
			fontWeight = weight;
			fontWeightOpen = false;
		};
	}

	function setBg(e: CustomEvent<string>) {
		if (videoExtensions.includes(e.detail.split('.').pop() ?? '')) {
			backgroundType = 'Video';
		} else {
			backgroundType = 'Image';
		}
		backgroundURL = e.detail;
		bgPickerOpen = false;
	}

	async function handleSave() {
		loading = true;
		invoke('save_template', {
			opts: {
				id: templateId,
				fontSize,
				fontWeight,
				name: templateName,
				backgroundUrl: backgroundURL,
				backgroundType
			}
		})
			.then((res) => {
				console.log({ res });
				loading = false;
			})
			.catch((err) => {
				console.error(err);
				loading = false;
			});
	}
</script>

<form
	on:submit|preventDefault={handleSave}
	class="p-3 border border-black/10 dark:border-white/10 h-fit rounded flex flex-col gap-2"
>
	<Input bind:value={templateName} placeholder="Template Name" class="rounded" required />
	<div class="flex gap-3">
		<P weight="semibold" class="whitespace-nowrap">Font Size</P>
		<Input bind:value={fontSize} class="rounded w-20 py-1" size="sm" type="number" />
	</div>
	<div class="flex gap-3">
		<P weight="semibold" class="whitespace-nowrap">Font Weight</P>
		<Button
			class="rounded py-1 bg-black/10 dark:bg-white/20 font-medium"
			color="alternative"
			size="xs"
			on:click={() => (fontWeightOpen = !fontWeightOpen)}
		>
			{fontWeight}
		</Button>
		<Dropdown transition={slide} open={fontWeightOpen}>
			<DropdownItem on:click={weighFactory('Light')}>Light</DropdownItem>
			<DropdownItem on:click={weighFactory('Normal')}>Normal</DropdownItem>
			<DropdownItem on:click={weighFactory('SemiBold')}>SemiBold</DropdownItem>
			<DropdownItem on:click={weighFactory('Bold')}>Bold</DropdownItem>
			<DropdownItem on:click={weighFactory('ExtraBold')}>ExtraBold</DropdownItem>
		</Dropdown>
	</div>
	<div class="w-full">
		<P size="xs" weight="medium">Background</P>
		<Button
			class="rounded py-1 bg-black/10 dark:bg-white/20 font-medium w-full p-0 overflow-hidden"
			color="alternative"
			size="xs"
			on:click={() => (bgPickerOpen = !bgPickerOpen)}
		>
			{#if backgroundURL}
				<img
					class="w-full max-w-[167px] aspect-video h-full object-cover"
					src={backgroundType === 'Video'
						? convertFileSrc(backgroundURL, { thumbnail: true })
						: convertFileSrc(backgroundURL)}
					alt="template background"
				/>
			{:else}
				<div class="w-full aspect-video flex items-center justify-center">None</div>
			{/if}
		</Button>
	</div>
	<Button color="green" type="submit" class="rounded">
		{#if loading}
			<Spinner />
		{:else}
			Save
		{/if}
	</Button>
</form>

<Modal class="pt-5" transition={scale} on:close={() => (bgPickerOpen = false)} open={bgPickerOpen}>
</Modal>
