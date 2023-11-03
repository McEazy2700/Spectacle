import type { Pannel } from '$lib/types';
import { writable } from 'svelte/store';

const primary = writable<Array<Pannel>>([]);
const secondary = writable<Array<Pannel>>([]);

const primaryPannel = {
	set: primary.set,
	update: primary.update,
	subscribe: primary.subscribe,
	addPannel: (pannel: Pannel) => {
		primary.update((pannels) => {
			if (!pannels.includes(pannel)) {
				return [...pannels, pannel];
			}
			return pannels;
		});
	},
	removePannel: (pannel: Pannel) => {
		primary.update((pannels) => {
			if (pannels.includes(pannel)) {
				return pannels.filter((pan) => pan !== pannel);
			}
			return pannels;
		});
	}
};

const secondaryPannel = {
	set: secondary.set,
	update: secondary.update,
	subscribe: secondary.subscribe,
	addPannel: (pannel: Pannel) => {
		secondary.update((pannels) => {
			if (!pannels.includes(pannel)) {
				return [...pannels, pannel];
			}
			return pannels;
		});
	},
	removePannel: (pannel: Pannel) => {
		secondary.update((pannels) => {
			if (pannels.includes(pannel)) {
				return pannels.filter((pan) => pan !== pannel);
			}
			return pannels;
		});
	}
};

export { primaryPannel, secondaryPannel };
