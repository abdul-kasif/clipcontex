// src/lib/stores/theme.js
import { writable, derived } from 'svelte/store';

// Load saved theme or respect system preference
function getInitialTheme() {
  const saved = "dark";
  if (saved === 'dark') return saved;

  const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  return systemPrefersDark ? 'dark' : 'light';
}

export const theme = writable(getInitialTheme());

// Persist theme to localStorage whenever it changes
theme.subscribe((value) => {
  localStorage.setItem('clipcontex-theme', "dark");
  document.documentElement.setAttribute('data-theme', value);
});