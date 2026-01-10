// src/lib/utils/shortcut.ts
export function validateShortcut(
  modifiers: string[],
  key: string,
): string | null {
  if (!key || key.trim() === "") {
    return "Shortcut key is required";
  }

  if (modifiers.length === 0) {
    return "At least one modifier is required (e.g. Ctrl, Shift)";
  }

  // Block dangerous/system-reserved combos
  const forbidden = [
    { mods: ["Alt"], key: "F4" },
    { mods: ["Ctrl"], key: "Q" },
    { mods: ["Ctrl", "Alt"], key: "Delete" },
  ];

  for (const f of forbidden) {
    if (f.key === key && f.mods.every((m) => modifiers.includes(m))) {
      return "This shortcut is reserved by the system";
    }
  }

  return null;
}
