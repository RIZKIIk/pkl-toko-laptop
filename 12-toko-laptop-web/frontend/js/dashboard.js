// Memuat data dashboard yang hanya tersedia setelah autentikasi.
import { ApiError, api, app, esc, html, reportError, state } from "./shared.js";
import { renderLogin } from "./login.js";
import { normalizeLaptop, renderDashboard } from "./dashboard-view.js";

export async function loadLaptops() {
  const result = await api("/api/laptops");
  state.laptops = (result.data || []).map(normalizeLaptop);
}

// Memastikan session valid sebelum memuat data dashboard.
export async function openDashboard() {
  if (location.pathname !== "/dashboard") {
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
    await loadLaptops();
    renderDashboard();
  } catch (error) {
    if (!(error instanceof ApiError && error.status === 401))
      reportError(error);
    if (error.status !== 401) {
      app.innerHTML = html`<div class="error-state error-state-page">
        <strong>Dashboard belum dapat dimuat</strong>
        <p>${esc(error.message)}</p>
        <button class="secondary-button" data-action="retry">Coba lagi</button>
      </div>`;
    }
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
