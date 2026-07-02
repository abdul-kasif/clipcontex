import { invoke } from "@tauri-apps/api/core";

export async function setDragging(isDragging: boolean): Promise<void> {
  await invoke("set_dragging", { isDragging });
}
