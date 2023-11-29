<script lang="ts">
	import { BibleBookChapterVerses, BibleBookChapters, BibleBooksList, BibleVersion } from '.';
	import type { ScriptureResult } from '$lib/models/scripture';
	import { invoke } from '@tauri-apps/api';
	import activeScripture from '$lib/stores/activeScripture';
	import scriptures from '$lib/stores/scriptures';


	$: {
		if ($scriptures.length > 0) {
			if (!$activeScripture.verseId) {
				activeScripture.update((curr) => {
					return {
						...curr,
						verseId: $scriptures[0].verse_id,
						nextVerseId: $scriptures[1].verse_id
					};
				});
			}
		}
	}

	$: {
		invoke('get_scriptures', {
			opts: {
				version: $activeScripture.bible.id,
				book: $activeScripture.book.abrv,
				chapter: $activeScripture.chapter
			}
		}).then((res) => {
			scriptures.set(res as ScriptureResult[]);
		});
	}
</script>

<div class="flex min-h-full items-center gap-4">
	<div class="flex flex-col gap-2">
		<div>
			<BibleVersion
				on:select={(e) => activeScripture.update((curr) => ({ ...curr, bible: e.detail }))}
				activeBibleVersion={$activeScripture.bible.version}
			/>
		</div>
		<div class="flex items-start gap-1">
      <BibleBooksList />
      <BibleBookChapters />
      <BibleBookChapterVerses />
		</div>
	</div>
</div>
