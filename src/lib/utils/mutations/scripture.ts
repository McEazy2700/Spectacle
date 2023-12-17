import { BIBLE_BOOKS } from "$lib/constants/bible";
import type { ScriptureResult } from "$lib/models/scripture";
import type { ScriptureSlide, View } from "$lib/models/slide";
import activeScripture from "$lib/stores/active/scripture";
import activeTemplate from "$lib/stores/active/template";
import liveView from "$lib/stores/views/liveView";
import { setLiveView } from "./live";

interface SetLiveViewOpts {
  scripture: ScriptureResult;
  text: string;
  templateId: number;
}

export async function setLiveScriptureView(opts: SetLiveViewOpts) {
  const scripture = opts.scripture;
  const books = BIBLE_BOOKS.filter((b) =>
    b.abrv.toUpperCase() == scripture.book
  );
  const bookName = books.pop()?.name;
  const passage = `${bookName} ${scripture.chapter}:${scripture.start_verse}`;

  const scriptureSlide: ScriptureSlide = {
    Scripture: {
      id: scripture.verse_id,
      text: opts.text,
      type: "Scripture",
      passage,
      template_id: opts.templateId,
    },
  };

  const view: View = {
    id: scripture.verse_id,
    type: "Scripture",
    name: passage,
    slide: scriptureSlide,
  };

  setLiveView(view, () => {
    activeScripture.update((curr) => ({
      ...curr,
      verseId: scripture.verse_id,
    }));
    activeTemplate.set(opts.templateId);
    liveView.set("Scripture");
  });
}
