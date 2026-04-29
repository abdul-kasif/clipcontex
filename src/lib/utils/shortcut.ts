const ALLOWED_MODIFIERS = new Set(["Ctrl", "Shift", "Alt", "Meta"]);
const ALLOWED_KEYS = new Set([
  ..."ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".split(""),
  ...Array.from({ length: 12 }, (_, i) => `F${i + 1}`),
  "Enter",
  "Escape",
  "Space",
  "Tab",
  "Backspace",
  "Delete",
  "Insert",
  "Home",
  "End",
  "PageUp",
  "PageDown",
  "ArrowUp",
  "ArrowDown",
  "ArrowLeft",
  "ArrowRight",
]);

const FORBIDDEN_SHORTCUTS = [
  { mods: ["Alt"], key: "F4" },
  { mods: ["Ctrl"], key: "Q" },
  { mods: ["Ctrl", "Alt"], key: "Delete" },
  { mods: ["Ctrl", "Shift"], key: "Escape" },
  { mods: ["Meta"], key: "Q" },
  { mods: ["Meta", "Shift"], key: "Q" },
];

export function validateShortcut(
  rawModifiers: string[],
  rawKey: string,
): string | null {
  const modifiers = rawModifiers.map((m) => m.trim()).filter((m) => m !== "");
  const key = rawKey.trim();

  if (!key) {
    return "Please choose a key for your shortcut.";
  }

  if (!ALLOWED_KEYS.has(key)) {
    return "This key is not supported for custom shortcuts. Please pick a standard key (e.g., A–Z, 0–9, F1–F12, Enter, etc.).";
  }

  if (modifiers.length === 0) {
    return "Add at least one modifier (Ctrl, Shift, Alt, or Cmd) to avoid conflicts with normal typing.";
  }

  const invalidModifier = modifiers.find((m) => !ALLOWED_MODIFIERS.has(m));
  if (invalidModifier) {
    return `“${invalidModifier}” is not a valid modifier. Use Ctrl, Shift, Alt, or Cmd (Meta).`;
  }

  const normalizedMods = [...new Set(modifiers)].sort();

  for (const { mods, key: forbiddenKey } of FORBIDDEN_SHORTCUTS) {
    const normalizedForbiddenMods = [...mods].sort();
    if (
      forbiddenKey === key &&
      normalizedMods.length === normalizedForbiddenMods.length &&
      normalizedMods.every((m, i) => m === normalizedForbiddenMods[i])
    ) {
      return "This shortcut is reserved by your operating system and can’t be used.";
    }
  }

  return null;
}
