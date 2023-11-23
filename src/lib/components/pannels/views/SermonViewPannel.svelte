<!-- <script lang="ts"> -->
<!-- 	import { ViewPannel } from '.'; -->
<!-- 	import { emit } from '@tauri-apps/api/event'; -->
<!-- 	import { Slide, IconButton } from '$lib/components/elements'; -->
<!-- 	import { TextForm } from '$lib/components/forms'; -->
<!-- 	import { NoSlides } from '$lib/components/empty'; -->
<!-- 	import { SermonSlidePreview } from '$lib/components/previews'; -->
<!-- 	import { slides } from '$lib/stores/views/sermon'; -->

<!-- 	let slideFormOpen = false; -->
<!-- 	let currSlide: number | undefined = undefined; -->
<!-- 	let formContent: string | undefined = undefined; -->

<!-- 	function handlePresent() { -->
<!-- 		if (currSlide !== undefined) { -->
<!-- 			let template = new Template(); -->
<!-- 			let sermon = new SermonView($slides[currSlide], template); -->
<!-- 			emit('presentation-update', { -->
<!-- 				view: new View(sermon, 'Sermon') -->
<!-- 			}); -->
<!-- 		} -->
<!-- 	} -->

<!-- 	function handleEditSlide(e: CustomEvent<{ id: number }>) { -->
<!-- 		currSlide = e.detail.id; -->
<!-- 		if ($slides[e.detail.id]) { -->
<!-- 			formContent = $slides[e.detail.id]; -->
<!-- 			slideFormOpen = true; -->
<!-- 		} -->
<!-- 	} -->

<!-- 	function handleSelectSlide(e: CustomEvent<{ id: number; html?: string }>) { -->
<!-- 		currSlide = e.detail.id; -->
<!-- 		formContent = $slides[e.detail.id]; -->
<!-- 		slideFormOpen = false; -->
<!-- 	} -->

<!-- 	function handleSlideFormSubmit(e: CustomEvent<string>) { -->
<!-- 		if (currSlide !== undefined) { -->
<!-- 			slides.updateSlide(currSlide, e.detail); -->
<!-- 		} else { -->
<!-- 			slides.addSlide(e.detail); -->
<!-- 		} -->
<!-- 		currSlide = $slides.length - 1; -->
<!-- 		slideFormOpen = false; -->
<!-- 	} -->

<!-- 	function handleNewSlide() { -->
<!-- 		slideFormOpen = true; -->
<!-- 		currSlide = undefined; -->
<!-- 		formContent = undefined; -->
<!-- 	} -->
<!-- </script> -->

<!-- <ViewPannel label="Sermon" disabled={currSlide === undefined} on:present={handlePresent}> -->
<!-- 	<div class="flex gap-2 flex-col md:flex-row"> -->
<!-- 		{#if $slides.length < 1} -->
<!-- 			<NoSlides /> -->
<!-- 		{:else} -->
<!-- 			<ul class="flex flex-col w-fit gap-1"> -->
<!-- 				{#each $slides as slide, id (id)} -->
<!-- 					<Slide active={currSlide == id} on:click={handleSelectSlide} slideId={id} html={slide} /> -->
<!-- 				{/each} -->
<!-- 			</ul> -->
<!-- 		{/if} -->
<!-- 		<div class="flex-1"> -->
<!-- 			{#if slideFormOpen || $slides.length < 1} -->
<!-- 				<TextForm -->
<!-- 					buttonName={currSlide !== undefined ? 'Save Slide' : 'Add Slide'} -->
<!-- 					bind:content={formContent} -->
<!-- 					on:submit={handleSlideFormSubmit} -->
<!-- 				/> -->
<!-- 			{:else} -->
<!-- 				<div class="flex flex-col gap-2"> -->
<!-- 					{#if currSlide !== undefined} -->
<!-- 						<SermonSlidePreview -->
<!-- 							on:edit={handleEditSlide} -->
<!-- 							bind:slideId={currSlide} -->
<!-- 							bind:content={$slides[currSlide]} -->
<!-- 						/> -->
<!-- 					{/if} -->
<!-- 					<IconButton variant="add" on:click={handleNewSlide} title="New Slide" /> -->
<!-- 				</div> -->
<!-- 			{/if} -->
<!-- 		</div> -->
<!-- 	</div> -->
<!-- </ViewPannel> -->
<div />
