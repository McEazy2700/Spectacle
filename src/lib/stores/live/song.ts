import type { SongData } from "$lib/models/song";
import { writable } from "svelte/store";

export interface LiveSong {
  templateId: number
  liveVerseIndex: number
  song: SongData
}

const liveSong = writable<LiveSong | undefined>()
export default liveSong
