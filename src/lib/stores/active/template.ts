import { writable } from "svelte/store";

const activeTemplate = writable<number>();

export default activeTemplate;
