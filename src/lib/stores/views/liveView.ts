import type { View } from "$lib/models/slide";
import { writable } from "svelte/store";

const liveView = writable<View | undefined>(undefined);

export default liveView
