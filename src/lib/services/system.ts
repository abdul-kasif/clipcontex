import { invoke } from "@tauri-apps/api/core";

export async function checkKdotoolInstalled(): Promise<boolean> {
  const response = await invoke("check_kdotool_installed");
  return response === true;
}

export async function setDragging(isDragging: boolean): Promise<void> {
  await invoke("set_dragging", { isDragging });
}
