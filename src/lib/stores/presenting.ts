import { writable } from 'svelte/store';

const presenting = writable<boolean>(false);

export default presenting;
