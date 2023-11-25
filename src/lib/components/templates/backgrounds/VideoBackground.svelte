<script lang="ts">
	import { convertFileSrc } from '$lib/utils/media';

	export let src: string;
</script>

<video class="w-full aspect-video object-cover" autoplay loop muted src={convertFileSrc(src)}>
	<track kind="captions" />
	<source src={convertFileSrc(src).trimEnd()} />
</video>
