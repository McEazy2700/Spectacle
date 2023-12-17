import type { ScriptureResult } from "$lib/models/scripture";
import { writable } from "svelte/store";

const scriptures = writable<ScriptureResult[]>([])
export default scriptures;
