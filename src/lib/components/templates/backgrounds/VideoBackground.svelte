<script lang="ts">
	import { convertFileSrc } from '$lib/utils/media';

	export let src: string;
  export let preview = false
</script>

<video
	class="min-w-full min-h-full aspect-video object-cover"
	autoplay
	loop
	muted
>
	<track kind="captions" />
	<source src={preview ? convertFileSrc(src, { thumbnail: true } ): convertFileSrc(src)} />
</video>
