// Memuat data dashboard yang hanya tersedia setelah autentikasi.
import { ApiError, api, app, esc, html, reportError, state } from "./shared.js";
import { renderLogin } from "./login.js";
import {
  normalizeLaptop,
  renderDashboard,
  renderProfilePage,
} from "./dashboard-view.js";
import { openCreateUserPage } from "./user-actions.js";

export async function loadLaptops() {
  const result = await api("/api/laptops");
  state.laptops = (result.data || []).map(normalizeLaptop);
}

// Memastikan session valid sebelum memuat data dashboard.
export async function openDashboard() {
  const isCreateUserPage = location.pathname === "/dashboard/users/new";
  const isProfilePage = location.pathname === "/dashboard/profile";
  if (
    location.pathname !== "/dashboard" &&
    !isCreateUserPage &&
    !isProfilePage
  ) {
    try {
      await api("/api/auth/session");
      location.replace("/dashboard");
    } catch (_) {
      renderLogin();
    }
    return;
  }
  try {
    const result = await api("/api/auth/session");
    state.user = result.data;
    if (isCreateUserPage) {
      if (state.user.role !== "admin") {
        location.replace("/dashboard");
        return;
      }
      await openCreateUserPage();
      return;
    }
    if (isProfilePage) {
      renderProfilePage();
      return;
    }
    await loadLaptops();
    renderDashboard();
  } catch (error) {
    if (error instanceof ApiError && error.status === 401) {
      renderLogin();
      return;
    }
    reportError(error);
    app.innerHTML = html`<div class="error-state error-state-page">
      <strong>${isCreateUserPage
        ? "Halaman tambah akun belum dapat dimuat"
        : isProfilePage
          ? "Profil belum dapat dimuat"
          : "Dashboard belum dapat dimuat"}</strong>
      <p>${esc(error.message)}</p>
      <button class="secondary-button" data-action="${isCreateUserPage
        ? "retry-account"
        : "retry"}">Coba lagi</button>
    </div>`;
  }
}

export async function refreshDashboard() {
  try {
    await loadLaptops();
    renderDashboard();
  } catch (error) {
    reportError(error);
  }
}
