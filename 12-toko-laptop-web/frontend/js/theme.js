// Tombol tema dan preferensi light/dark yang dipakai semua halaman.
import { html, icons } from "./shared.js";

const THEME_KEY = "laptopku.theme";

function temaAktif() {
  return document.documentElement.dataset.theme === "dark" ? "dark" : "light";
}

export function themeToggleMarkup() {
  const dark = temaAktif() === "dark";
  const label = dark ? "Mode terang" : "Mode gelap";
  return html`<button
    class="theme-toggle"
    type="button"
    data-action="theme-toggle"
    aria-pressed="${dark}"
    aria-label="Ubah ke ${dark ? "mode terang" : "mode gelap"}"
    title="Ubah ke ${dark ? "mode terang" : "mode gelap"}"
  >
    <span class="theme-toggle-icon" aria-hidden="true">
      <span class="theme-icon-light">${icons.moon}</span>
      <span class="theme-icon-dark">${icons.sun}</span>
    </span>
    <span class="theme-toggle-label">${label}</span>
  </button>`;
}

// Simpan preferensi, lalu perbarui semua tombol tema yang sedang tampil.
export function toggleTheme() {
  const nextTheme = temaAktif() === "dark" ? "light" : "dark";
  document.documentElement.dataset.theme = nextTheme;
  document
    .querySelector('meta[name="theme-color"]')
    ?.setAttribute("content", nextTheme === "dark" ? "#101719" : "#f3f6f6");

  try {
    localStorage.setItem(THEME_KEY, nextTheme);
  } catch (_) {
    // Tema tetap berubah sampai halaman ditutup jika storage tidak tersedia.
  }

  const dark = nextTheme === "dark";
  const label = dark ? "Mode terang" : "Mode gelap";
  const actionLabel = `Ubah ke ${label.toLowerCase()}`;
  document.querySelectorAll('[data-action="theme-toggle"]').forEach((button) => {
    button.setAttribute("aria-pressed", String(dark));
    button.setAttribute("aria-label", actionLabel);
    button.title = actionLabel;
    button.querySelector(".theme-toggle-label").textContent = label;
  });
}
