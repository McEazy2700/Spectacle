import { BIBLE_BOOKS } from "$lib/constants/bible";
import type { ScriptureResult } from "$lib/models/scripture";
import type { ActiveScripture } from "$lib/stores/active/scripture";
import activeScripture from "$lib/stores/active/scripture";
import scriptures from "$lib/stores/lists/scriptures";
import { invoke } from "@tauri-apps/api";

export const getPreviousScripture = async (
  aScript: ActiveScripture,
  scriptures: ScriptureResult[],
): Promise<ScriptureResult> => {
  const curr = scriptures.filter((s) => s.verse_id === aScript.verseId).pop();
  const currBook =
    BIBLE_BOOKS.filter((b) =>
      b.abrv.toUpperCase() === curr?.book.toUpperCase()
    )[0];

  // Get Previous Verse
  const verseId = parseInt(curr?.start_verse ?? `${scriptures.length - 1}`) - 1;
  if (!(verseId < 1)) {
    return scriptures.filter((s) => (s.start_verse === `${verseId}`))[0];
  }

  // Get Previous Chapter
  const chapterId = parseInt(curr?.chapter ?? `${currBook.chapters}`) - 1;
  if (!(chapterId < 1)) {
    const newScriptures = await fetchScriptures(
      aScript,
      aScript.book.abrv,
      chapterId,
    );
    activeScripture.update((c) => ({ ...c, chapter: chapterId }));
    const lVerse = `${newScriptures.length}`;
    return newScriptures.filter((s) => (s.start_verse === lVerse))[0];
  }

  // Get Next Book
  const prevBookIndex = BIBLE_BOOKS.indexOf(currBook) - 1;
  const bookIndex = prevBookIndex < 0 ? BIBLE_BOOKS.length - 1 : prevBookIndex;
  const prevBook = BIBLE_BOOKS[bookIndex];
  const newScriptures = await fetchScriptures(
    aScript,
    prevBook.abrv,
    prevBook.chapters,
  );
  activeScripture.update((c) => ({
    ...c,
    chapter: prevBook.chapters,
    verse: newScriptures.length,
    book: prevBook,
  }));
  return newScriptures.filter((s) =>
    s.start_verse === `${newScriptures.length}`
  )[0];
};

export const getNextScripture = async (
  aScript: ActiveScripture,
  scriptures: ScriptureResult[],
): Promise<ScriptureResult> => {
  const curr = scriptures.filter((s) => s.verse_id === aScript.verseId).pop();

  // Get Next Verse
  const verseId = `${parseInt(curr?.start_verse ?? "1") + 1}`;
  let nextVerse = scriptures.filter((s) => (s.start_verse === verseId)).pop();
  if (nextVerse) {
    return nextVerse;
  }

  // Get Next Chapter
  const nextChapterNumber = parseInt(curr?.chapter ?? "1") + 1;
  let newScriptures = await fetchScriptures(
    aScript,
    aScript.book.abrv,
    nextChapterNumber,
  );

  nextVerse = newScriptures.filter((s) => (s.start_verse === "1")).pop();
  activeScripture.update((c) => ({ ...c, chapter: nextChapterNumber }));
  if (nextVerse) {
    return nextVerse;
  }

  // Get Next Book
  const currBook =
    BIBLE_BOOKS.filter((b) =>
      b.abrv.toUpperCase() === curr?.book.toUpperCase()
    )[0];

  const nextBookNumber = BIBLE_BOOKS.indexOf(currBook) + 1;
  const bookIndex = nextBookNumber <= BIBLE_BOOKS.length - 1
    ? nextBookNumber
    : 0;
  const newBook = BIBLE_BOOKS[bookIndex];
  newScriptures = await fetchScriptures(aScript, newBook.abrv, 1);
  activeScripture.update((c) => ({
    ...c,
    chapter: 1,
    verse: 1,
    book: newBook,
  }));
  return newScriptures.filter((s) => s.start_verse === "1")[0];
};

const fetchScriptures = async (
  s: ActiveScripture,
  book: string,
  chapter: number,
): Promise<ScriptureResult[]> => {
  const res = await invoke("get_scriptures", {
    opts: {
      version: s.bible.id,
      book,
      chapter,
    },
  });

  const scripts = res as ScriptureResult[];
  scriptures.set(scripts);
  return scripts;
};
