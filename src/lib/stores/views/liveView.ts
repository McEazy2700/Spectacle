import type { View } from "$lib/classes/views";
import { writable } from "svelte/store";

const liveView = writable<View | undefined>(undefined);

export default liveView
