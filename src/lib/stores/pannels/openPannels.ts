import { writable } from "svelte/store";
import type { Pannel } from "$lib/models/pannels";

const { update, set, subscribe } = writable<Pannel[]>([
  "Preview",
  "Schedule",
  "LiveView",
  "Explorer",
]);

function remove(pannel: Pannel) {
  update((curr) => {
    const updated = curr.filter((item) => item !== pannel);
    return updated;
  });
}

function add(pannel: Pannel) {
  update((curr) => {
    if (!curr.includes(pannel)) {
      return [...curr, pannel];
    }
    return curr;
  });
}

export default {
  set,
  subscribe,
  update,
  remove,
  add,
};
