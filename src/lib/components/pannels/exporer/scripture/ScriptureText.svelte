<script lang="ts">
	import type { ScriptureResult } from '$lib/models/scripture';

	export let scripture: ScriptureResult;
	$: text = new TextDecoder('utf-8').decode(new Uint8Array(scripture.verse_text as any));
</script>

<button class="flex items-start gap-1 group">
	<span
		class={`
      p-4 rounded border border-black/10 dark:border-white/10 h-full flex items-center
      justify-center group-hover:bg-primary-600 transition-all group-hover:text-white
    `}
	>
		{scripture.start_verse}
	</span>
	<span title={text}
		class="p-2 border border-black/10 dark:border-white/10 rounded h-full w-full group-hover:bg-primary-600 transition-all group-hover:text-white"
	>
		<span class="line-clamp-2 text-start">
			{text}
		</span>
	</span>
</button>
