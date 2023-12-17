<script lang="ts">
	/* import schedule from '$lib/stores/schedule'; */
	/*  import type { ScriptureSlide } from '$lib/models/slide';} */
	import { NoSlides } from '$lib/components/slides';
	import { Pannel } from '$lib/components/pannels';
</script>

<Pannel class="max-w-[15rem] w-full">
	<ul class="w-full grid grid-cols-1 gap-2 p-1 max-w-xs">
		<NoSlides />
	</ul>
</Pannel>

<style>
</style>
