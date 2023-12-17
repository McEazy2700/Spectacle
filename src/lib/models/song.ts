export interface VerseData {
  label: string;
  type: 'v' | 'c';
  text: string;
}

export interface SongData {
  title?: string;
  version: string;
  verses: VerseData[];
}
export type SongFormatType = "OpenLp"


export interface SongOptions {
  format: SongFormatType,
  lang: string;
  offset?: number;
  limit?: number;
  search_title?: string;
  search_text?: string;
}

export interface SongModel {
  id: number
  song_book_id?: number
  title: string
  alternate_title?: string
  lyrics: string
  verse_order?: string
  copyright?: string
  comments?: string
  ccli_number?: string
  song_number?: string
  theme_name?: string
  search_title: string
  search_lyrics: string
  create_date?: string
  last_modified?: string
  temporary?: boolean
}

export interface SongUpdateOptions {
    id: number,
    format: SongFormatType,
    lang: string,
    title?: string,
    lyrics?: string,
}
