import type { View } from "$lib/models/slide";
import alerts from "$lib/stores/alerts";
import { invoke } from "@tauri-apps/api";

export const setLiveView = async (view: View, onSuccess?: () => void) => {
  try {
    await invoke("set_live_view", { view: view });
    onSuccess && onSuccess()
  } catch (err) {
    alerts.add({ message: `Error: ${err}`, kind: "error" });
  }
}
