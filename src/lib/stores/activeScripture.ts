import { BIBLE_BOOKS, BIBLE_VERSIONS, type BibleVersion } from "$lib/constants/bible";
import { writable } from "svelte/store";

export interface ActiveScripture {
  book: {
    name: string;
    abrv: string;
    chapters: number;
  },
  bible: BibleVersion,
  chapter: number,
  verse: number,
  verseId?: string,
  nextVerseId?: string,
}

const activeScripture = writable<ActiveScripture>({
  book: BIBLE_BOOKS[0],
  bible: BIBLE_VERSIONS[0],
  chapter: 1,
  verse: 1
})
export default activeScripture;
