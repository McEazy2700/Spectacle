import type { TemplateView } from "$lib/models/slide";
import { writable } from "svelte/store";

const liveView = writable<TemplateView | undefined>(undefined);

export default liveView
