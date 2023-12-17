export interface OpenLpSongLang {
  language: string;
  url: string;
  songCount: number
}

export const OPENLP_SONGS: OpenLpSongLang[] = [
  { language: "English", url: "https://worshipleaderapp.com/download/openlp3/en.sqlite", songCount: 3000 },
  { language: "French", url: "https://worshipleaderapp.com/download/openlp3/fr.sqlite", songCount: 2305 }
]
