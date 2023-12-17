import { writable } from "svelte/store";

type Previewable = "Song" | "Scripture" | "Text" | "Media"

const activePreview = writable<Previewable | undefined>(undefined)
export default activePreview
