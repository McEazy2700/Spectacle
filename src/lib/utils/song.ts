import type { SlideType, SongSlide, View } from "$lib/models/slide";
import type { SongData, VerseData } from "$lib/models/song";
import type { LiveSong } from "$lib/stores/live/song";
import liveSong from "$lib/stores/live/song";
import liveView from "$lib/stores/views/liveView";
import { setLiveView } from "./mutations/live";

export function parseSongXML(xml: string, title?: string): SongData {
  const xDoc = new DOMParser().parseFromString(xml, "application/xml");
  const songVersion = xDoc.querySelector("song")?.getAttribute("version") ?? "";
  const versesDoc = xDoc.querySelectorAll("verse");
  const verses: VerseData[] = [];

  for (const ver of versesDoc) {
    const label = ver.getAttribute("label") ?? "";
    const typ = ver.getAttribute("type");
    const text = ver.textContent?.trim() ?? "";
    verses.push({ label, type: typ as "v" | "c", text });
  }
  const song: SongData = {
    title,
    version: songVersion,
    verses,
  };
  return song;
}

export const stringifySongToXML = (song: SongData): string => {
  let verses = "";

  for (const verse of song.verses) {
    verses +=
      `<verse label="${verse.label}" type="${verse.type}"><![CDATA[${verse.text}]]></verse>`;
  }
  return `<?xml version='1.0' encoding='UTF-8'?><song version="${song.version}"><lyrics>${verses}</lyrics></song>`;
};

export const makeSlide = (verse: VerseData, templateId: number) => {
  const slide: SongSlide = {
    Song: {
      id: verse.label,
      type: "Song",
      verse: parseInt(verse.label),
      text: verse.text,
      template_id: templateId,
    },
  };
  return slide;
};

export const setNextLiveSongSlide = (current: LiveSong) => {
  if (current.liveVerseIndex < current.song.verses.length - 1) {
    const currId = current.liveVerseIndex;
    const slide = makeSlide(
      current.song.verses[currId + 1],
      current.templateId,
    );
    const view: View = {
      slide,
      name: current.song.title ?? "",
      type: "Song",
      id: `${currId + 1}`,
    };
    setLiveView(view, () => {
      liveSong.set({
        song: current.song,
        liveVerseIndex: currId + 1,
        templateId: current.templateId,
      });
      liveView.set("Song");
    });
  }
};


export const setPrevLiveSongSlide = (current: LiveSong) => {
  if (current.liveVerseIndex - 1 >= 0) {
    const currId = current.liveVerseIndex;
    const slide = makeSlide(
      current.song.verses[currId - 1],
      current.templateId,
    );
    const view: View = {
      slide,
      name: current.song.title ?? "",
      type: "Song",
      id: `${currId - 1}`,
    };
    setLiveView(view, () => {
      liveSong.set({
        song: current.song,
        liveVerseIndex: currId - 1,
        templateId: current.templateId,
      });
      liveView.set("Song");
    });
  }
};
