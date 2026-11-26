// State, helper API, ikon, dan komponen kecil yang dipakai bersama.
// Elemen utama yang dipakai bersama oleh halaman login dan dashboard.
export const app = document.querySelector("#app");
export const dialog = document.querySelector("#dialog");
const toastRegion = document.querySelector("#toast-region");

// State frontend. Token bertahan selama tab terbuka; cookie dikirim oleh browser.
const TOKEN_KEY = "toko-laptop.access-token";
export const state = {
  user: null,
  laptops: [],
  search: "",
  stockFilter: "all",
  productSort: "default",
  sidebarCollapsed: false,
};

// Tag helper supaya markup dinamis bisa diformat dengan aturan HTML.
export const html = String.raw;

// Ikon SVG dipusatkan agar komponen memakai nama yang konsisten.
export const icons = {
  laptop: html`<svg
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="1.8"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <rect x="4" y="4" width="16" height="12" rx="2" />
    <path d="M2 20h20l-1.4-2.5H3.4L2 20Z" />
  </svg>`,
  grid: html`<svg
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="1.8"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <rect x="3" y="3" width="7" height="7" rx="1.5" />
    <rect x="14" y="3" width="7" height="7" rx="1.5" />
    <rect x="3" y="14" width="7" height="7" rx="1.5" />
    <rect x="14" y="14" width="7" height="7" rx="1.5" />
  </svg>`,
  box: html`<svg
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="1.8"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <path d="m4 7 8-4 8 4-8 4-8-4Z" />
    <path d="m4 7 8 4 8-4M4 7v10l8 4 8-4V7M12 11v10" />
  </svg>`,
  trend: html`<svg
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="1.8"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <path d="M3 3v18h18" />
    <path d="m7 14 4-4 4 3 6-7" />
    <path d="M16 6h5v5" />
  </svg>`,
  plus: html`<svg
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    aria-hidden="true"
  >
    <path d="M12 5v14M5 12h14" />
  </svg>`,
  search: html`<svg
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="1.8"
    stroke-linecap="round"
    aria-hidden="true"
  >
    <circle cx="10.8" cy="10.8" r="6.8" />
    <path d="m16 16 5 5" />
  </svg>`,
  logout: html`<svg
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="1.8"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <path d="M10 17l5-5-5-5M15 12H3" />
    <path d="M12 3h6a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-6" />
  </svg>`,
  menu: html`<svg
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="1.8"
    stroke-linecap="round"
    aria-hidden="true"
  >
    <path d="M4 6h16M4 12h16M4 18h16" />
  </svg>`,
  sun: html`<svg
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="1.8"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <circle cx="12" cy="12" r="4" />
    <path d="M12 2v2m0 16v2M4.93 4.93l1.42 1.42m11.3 11.3 1.42 1.42M2 12h2m16 0h2M4.93 19.07l1.42-1.42m11.3-11.3 1.42-1.42" />
  </svg>`,
  moon: html`<svg
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="1.8"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <path d="M20.5 14.2A8.6 8.6 0 0 1 9.8 3.5 8.8 8.8 0 1 0 20.5 14.2Z" />
  </svg>`,
  eye: html`<svg
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="1.8"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <path d="M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7S2 12 2 12Z" />
    <circle cx="12" cy="12" r="3" />
  </svg>`,
  eyeOff: html`<svg
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="1.8"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <path d="m3 3 18 18M10.6 10.6a2 2 0 0 0 2.8 2.8" />
    <path
      d="M9.9 5.2A10.8 10.8 0 0 1 12 5c6.5 0 10 7 10 7a14 14 0 0 1-3 3.8M6.2 6.2C3.5 8 2 12 2 12s3.5 7 10 7a10 10 0 0 0 4.2-.9"
    />
  </svg>`,
  close: html`<svg
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    aria-hidden="true"
  >
    <path d="m6 6 12 12M18 6 6 18" />
  </svg>`,
};

// Error API menyimpan status HTTP untuk penanganan 401 dan 403.
export class ApiError extends Error {
  constructor(message, status) {
    super(message);
    this.status = status;
  }
}

export const token = () => sessionStorage.getItem(TOKEN_KEY);
export const saveToken = (value) =>
  value
    ? sessionStorage.setItem(TOKEN_KEY, value)
    : sessionStorage.removeItem(TOKEN_KEY);
export const esc = (value) =>
  String(value ?? "").replace(
    /[&<>"']/g,
    (char) =>
      ({
        "&": "&amp;",
        "<": "&lt;",
        ">": "&gt;",
        '"': "&quot;",
        "'": "&#39;",
      })[char],
  );
export const roleName = (role) =>
  ({ admin: "Administrator", buyer: "Pembeli", restocker: "Petugas Stok" })[
    role
  ] ||
  role ||
  "Pengguna";

// Semua request memakai cookie session dan bearer token bila tersedia.
export async function api(path, options = {}) {
  const headers = new Headers(options.headers || {});
  if (options.body && !headers.has("Content-Type"))
    headers.set("Content-Type", "application/json");
  if (token()) headers.set("Authorization", `Bearer ${token()}`);
  let response;
  try {
    response = await fetch(path, {
      ...options,
      headers,
      credentials: "include",
    });
  } catch (_) {
    throw new ApiError(
      "Tidak dapat terhubung ke server. Periksa koneksi lalu coba lagi.",
      0,
    );
  }
  let body = {};
  try {
    body = await response.json();
  } catch (_) {
    /* response tanpa JSON */
  }
  if (response.status === 401 && options.auth !== false) {
    saveToken(null);
    if (location.pathname !== "/login") {
      location.assign("/login?session=expired");
    }
  }
  if (!response.ok) {
    throw new ApiError(
      body.message || `Permintaan gagal (HTTP ${response.status}).`,
      response.status,
    );
  }
  return body;
}

export function toast(message, isError = false) {
  const node = document.createElement("div");
  node.className = `toast${isError ? " error" : ""}`;
  node.textContent = message;
  toastRegion.append(node);
  window.setTimeout(() => node.remove(), 4200);
}

export function reportError(error) {
  if (error.status === 403)
    toast(
      "Akses ditolak. Role akun ini tidak dapat menjalankan aksi tersebut.",
      true,
    );
  else toast(error.message || "Terjadi kesalahan. Silakan coba lagi.", true);
}

export function brand(extraClass = "") {
  return html`<div class="brand ${extraClass}">
    <span class="brand-mark">${icons.laptop}</span
    ><span class="brand-name">Laptop<span class="brand-accent">Ku</span></span>
  </div>`;
}
