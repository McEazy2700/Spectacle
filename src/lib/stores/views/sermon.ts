import type { SermonView } from '$lib/classes/views';
import { writable } from 'svelte/store';

const sermon = writable<SermonView | undefined>(undefined);
const { update, set, subscribe } = writable<Array<string>>([]);

function addSlide(content: string) {
	update((slides) => {
		return [...slides, content];
	});
}

function removeSlide(index: number) {
	update((slides) => {
		if (slides[index] !== undefined) {
			return slides.splice(index, 1);
		}
		return slides;
	});
}

function updateSlide(index: number, content: string) {
	update((slides) => {
		if (slides[index] !== undefined) {
			slides[index] = content;
			return slides;
		}
		return slides;
	});
}

const slides = {
	set,
	update,
	addSlide,
	subscribe,
	updateSlide,
	removeSlide
};

export { sermon, slides };
