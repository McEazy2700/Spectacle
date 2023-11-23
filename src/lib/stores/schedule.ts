import type { ScheduleType } from "$lib/models/slide";
import { writable } from "svelte/store";

const schedule = writable<ScheduleType>([])

export default schedule
