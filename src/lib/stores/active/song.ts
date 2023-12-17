import type { SongFormatType, SongModel } from "$lib/models/song";
import { writable } from "svelte/store";

interface ActiveSong {
  lang: string,
  format: SongFormatType,
  song: SongModel,
}

const activeSong = writable<ActiveSong | undefined>(undefined)
export default activeSong

interface SongEditing {
  lang?: string,
  format?: SongFormatType,
  song?: SongModel,
  activeVerseIndex: number
}

export const songEditing = writable<SongEditing | undefined>()
