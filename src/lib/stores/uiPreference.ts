import { LazyStore } from "@tauri-apps/plugin-store";

let store: LazyStore | null = null;

function getStore() {
  if (!store) {
    store = new LazyStore("ui-preferences.json");
  }
  return store;
}

export async function getBoolean(
  key: string,
  defaultValue: boolean,
): Promise<boolean> {
  const value = await getStore().get<boolean>(key);
  return value ?? defaultValue;
}

export async function setBoolean(key: string, value: boolean): Promise<void> {
  const store = getStore();
  await store.set(key, value);
  await store.save();
}
