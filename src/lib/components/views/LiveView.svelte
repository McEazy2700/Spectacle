<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
  import { Slide } from "$lib/components/slides"
	import type { SlideType } from '$lib/models/slide';

  let liveSlide: SlideType | undefined = undefined

  export let size: "xs" | "sm" | "md" | "lg" | "full" = "full"
  export let bordered = false;
	listen('state-update', (e) => {
	});
</script>

<Slide {bordered} {size} slide={liveSlide}/>
