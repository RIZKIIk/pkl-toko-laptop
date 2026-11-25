// Titik masuk browser untuk memilih halaman dan menghubungkan aksi UI.
import { api, app, dialog, reportError, saveToken, state } from "./shared.js";
import { renderLogin } from "./login.js";
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

// Delegasi event membuat tombol dari markup dinamis tetap berfungsi.

app.addEventListener("click", (event) => {
  const control = event.target.closest("[data-action]");
  if (!control) return;
  const action = control.dataset.action;
  if (action === "create") openCreate();
  if (action === "detail") openDetail(control.dataset.id);
  if (action === "edit") openEdit(control.dataset.id);
  if (action === "delete") deleteLaptop(control.dataset.id);
  if (action === "logout") logout();
  if (action === "refresh") refreshDashboard();
  if (action === "retry") openDashboard();
  if (action === "close-dialog") dialog.close();
  if (action === "menu") {
    document.querySelector("#sidebar")?.classList.toggle("open");
    document.querySelector("#sidebar-shade")?.classList.toggle("open");
  }
});
app.addEventListener("input", (event) => {
  if (event.target.id !== "search") return;
  state.search = event.target.value;
  const target = document.querySelector("#inventory-content");
  if (target) target.innerHTML = inventoryMarkup(state.laptops);
});
app.addEventListener("click", (event) => {
  if (event.target.id === "sidebar-shade") {
    document.querySelector("#sidebar")?.classList.remove("open");
    event.target.classList.remove("open");
  }
});
dialog.addEventListener("click", (event) => {
  if (event.target === dialog) dialog.close();
});

if (location.pathname === "/dashboard") openDashboard();
else renderLogin();
