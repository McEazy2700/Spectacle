<script lang="ts">
	import { TemplateWraper } from '$lib/components/templates';
	import type { TemplateType } from '$lib/types';
	import { P } from 'flowbite-svelte';

	export let template: TemplateType;
</script>

<button
	class="flex flex-col border rounded-md border-black/20 dark:border-white/20"
>
	<div class="w-screen max-w-[200px] aspect-video">
		<TemplateWraper bind:template>Sample Text</TemplateWraper>
	</div>
	<P class="p-1 border-t text-center border-black/20 dark:border-white/30">{template.name}</P>
</button>
