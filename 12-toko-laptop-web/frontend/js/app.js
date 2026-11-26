// Titik masuk browser untuk memilih halaman dan menghubungkan aksi UI.
import { api, app, dialog, reportError, saveToken, state } from "./shared.js";
import { renderLogin } from "./login.js";
import { renderRegister } from "./register.js";
import { initSidebarControls } from "./sidebar.js";
import { toggleTheme } from "./theme.js";
import { openDashboard, refreshDashboard } from "./dashboard.js";
import { inventoryMarkup } from "./dashboard-view.js";
import {
  deleteLaptop,
  openCreate,
  openDetail,
  openEdit,
} from "./laptop-actions.js";

async function logout() {
  const button = document.querySelector('[data-action="logout"]');
  if (button) button.disabled = true;
  try {
    await api("/api/auth/logout", { method: "POST" });
  } catch (error) {
    if (error.status !== 401) reportError(error);
  }
  saveToken(null);
  state.user = null;
  location.assign("/login");
}

initSidebarControls();

// Dialog berada di luar #app, jadi seluruh aksi tombol didelegasikan ke document.
document.addEventListener("click", (event) => {
  if (!(event.target instanceof Element)) return;
  const control = event.target.closest("[data-action]");
  if (!control) return;
  const action = control.dataset.action;
  if (action === "create") openCreate();
  if (action === "detail") openDetail(control.dataset.id);
  if (action === "edit") openEdit(control.dataset.id);
  if (action === "delete") deleteLaptop(control.dataset.id);
  if (action === "logout") logout();
  if (action === "theme-toggle") toggleTheme();
  if (action === "refresh") refreshDashboard();
  if (action === "retry") openDashboard();
  if (action === "clear-filters") {
    state.search = "";
    state.stockFilter = "all";
    state.productSort = "default";
    const search = document.querySelector("#search");
    const stockFilter = document.querySelector("#stock-filter");
    const productSort = document.querySelector("#product-sort");
    if (search) search.value = "";
    if (stockFilter) stockFilter.value = "all";
    if (productSort) productSort.value = "default";
    const target = document.querySelector("#inventory-content");
    if (target) target.innerHTML = inventoryMarkup(state.laptops);
    search?.focus({ preventScroll: true });
  }
  if (action === "close-dialog") dialog.close();
});
app.addEventListener("input", (event) => {
  if (event.target.id !== "search") return;
  state.search = event.target.value;
  const target = document.querySelector("#inventory-content");
  if (target) target.innerHTML = inventoryMarkup(state.laptops);
});
app.addEventListener("change", (event) => {
  if (event.target.id === "stock-filter")
    state.stockFilter = event.target.value;
  else if (event.target.id === "product-sort")
    state.productSort = event.target.value;
  else return;

  const target = document.querySelector("#inventory-content");
  if (target) target.innerHTML = inventoryMarkup(state.laptops);
});
dialog.addEventListener("click", (event) => {
  if (event.target === dialog) dialog.close();
});

if (location.pathname === "/dashboard") openDashboard();
else if (location.pathname === "/register") renderRegister();
else renderLogin();
