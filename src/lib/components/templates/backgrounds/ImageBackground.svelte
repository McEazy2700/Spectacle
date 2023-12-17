<script lang="ts">
	import { convertFileSrc } from "$lib/utils/media";

	export let src: string;
</script>

<img class="min-w-full min-h-full aspect-video object-cover" src={convertFileSrc(src)} alt="background" />
