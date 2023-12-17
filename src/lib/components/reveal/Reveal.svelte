<script lang="ts">
	import Reveal from 'reveal.js';
	import HighLight from 'reveal.js/plugin/highlight/highlight';
	import { onMount } from 'svelte';
	import { ReavealSlide as Slide } from '.';

	import 'reveal.js/dist/reveal.css';
	import 'reveal.js/dist/theme/dracula.css';

	onMount(() => {
		const deck = new Reveal({
			autoAnimateEasing: 'ease',
			autoAnimateDuration: 0.3
		});
		deck.initialize({ plugins: [HighLight], embedded: true, disableLayout: true });
	});

	const R = {
		Slide
	};
</script>

<div class="reveal">
	<div class="slides">
		<slot {R} />
	</div>
</div>
