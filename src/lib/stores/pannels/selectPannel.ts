import type { Pannel } from '$lib/types';
import { writable } from 'svelte/store';

const selectPannel = writable<Pannel | undefined>(undefined);

export default selectPannel;
