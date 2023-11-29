<script lang="ts">
	import { BIBLE_BOOKS } from '$lib/constants/bible';
	import type { ScriptureResult } from '$lib/models/scripture';
	import type { ScriptureSlide, View } from '$lib/models/slide';
	import activeScripture from '$lib/stores/activeScripture';
	import alerts from '$lib/stores/alerts';
	import scriptures from '$lib/stores/scriptures';
	import { invoke } from '@tauri-apps/api';

	export let scripture: ScriptureResult;
	export let templateId: number;
	export let index: number;
	$: text = new TextDecoder('utf-8').decode(new Uint8Array(scripture.verse_text as any));

	async function goLive() {
		const bookName = BIBLE_BOOKS.filter((bible) => bible.abrv.toUpperCase() == scripture.book).pop()
			?.name;
		const passage = `${bookName} ${scripture.chapter}:${scripture.start_verse}`;
		const scriptureSlide: ScriptureSlide = {
			Scripture: {
				id: scripture.verse_id,
				text,
				type: 'Scripture',
				passage,
				template_id: templateId
			}
		};
		const view: View = {
			id: scripture.verse_id,
			type: 'Scripture',
			name: passage,
			slide: scriptureSlide
		};
		try {
			await invoke('set_live_view', { view: view });
			activeScripture.update((curr) => ({
				...curr,
				verseId: scripture.verse_id,
				nextVerseId: $scriptures[index + 1].verse_id
			}));
		} catch (err) {
			alerts.add({ message: `Error: ${err}`, kind: 'error' });
		}
	}
</script>

<button type="button" on:dblclick={goLive} class="flex items-start gap-1 group">
	<span
		class={`
      ${$activeScripture.verseId === scripture.verse_id ? 'bg-primary-600 text-white' : ''}
      p-4 rounded border border-black/10 dark:border-white/10 h-full flex items-center
      justify-center group-hover:bg-primary-600 transition-all group-hover:text-white
    `}
	>
		{scripture.start_verse}
	</span>
	<span
		title={text}
		class={`
      ${$activeScripture.verseId === scripture.verse_id ? 'bg-primary-600 text-white' : ''}
      p-2 border border-black/10 dark:border-white/10 rounded h-full w-full
      group-hover:bg-primary-600 transition-all group-hover:text-white
    `}
	>
		<span class="line-clamp-2 text-start">
			{text}
		</span>
	</span>
</button>
