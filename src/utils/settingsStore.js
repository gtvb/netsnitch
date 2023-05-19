import { writable } from "svelte/store";

export const settingsStore = writable({
    delay: 3
});
