import { writable } from "svelte/store";

export interface AlertData {
  kind: "success" | "error" | "warning",
  message: string
}

const { update, set, subscribe } = writable<AlertData[]>([])

function add(alert: AlertData) {
  update((curr) => {
    return [...curr, alert]
  })
}

function remove(index: number) {
  update(curr => {
    let arr = curr;
    arr.splice(index, 1)
    return arr
  })
}

export default { update, add, remove, set, subscribe }
