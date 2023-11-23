import type { Pannel } from '$lib/models/pannels';
import { writable } from 'svelte/store';

interface ActivePannel {
	type: Pannel;
	id?: number;
}

const activePannel = writable<ActivePannel | undefined>(undefined);
export default activePannel;
