// Interaksi sidebar: menu aktif, drawer ponsel, dan mode ringkas desktop.
import { state } from "./shared.js";

const MOBILE_LAYOUT = "(max-width: 620px)";

function isMobileLayout() {
  return window.matchMedia(MOBILE_LAYOUT).matches;
}

function updateToggleLabel() {
  const button = document.querySelector('[data-action="menu"]');
  const sidebar = document.querySelector("#sidebar");
  if (!button || !sidebar) return;

  const mobile = isMobileLayout();
  if (!mobile) {
    sidebar.classList.remove("open");
    document.querySelector("#sidebar-shade")?.classList.remove("open");
  }
  const expanded = mobile
    ? sidebar.classList.contains("open")
    : !state.sidebarCollapsed;
  const label = mobile
    ? expanded
      ? "Tutup menu"
      : "Buka menu"
    : expanded
      ? "Kecilkan sidebar"
      : "Besarkan sidebar";

  button.setAttribute("aria-expanded", String(expanded));
  button.setAttribute("aria-label", label);
  button.title = label;
}

function closeMobileSidebar() {
  document.querySelector("#sidebar")?.classList.remove("open");
  document.querySelector("#sidebar-shade")?.classList.remove("open");
  updateToggleLabel();
}

function activateLink(link) {
  document.querySelectorAll(".nav-link").forEach((item) => {
    const active = item === link;
    item.classList.toggle("active", active);
    if (active) item.setAttribute("aria-current", "page");
    else item.removeAttribute("aria-current");
  });

  if (isMobileLayout()) closeMobileSidebar();
}

function toggleSidebar() {
  const layout = document.querySelector(".app-layout");
  const sidebar = document.querySelector("#sidebar");
  if (!layout || !sidebar) return;

  if (isMobileLayout()) {
    const isOpen = sidebar.classList.toggle("open");
    document.querySelector("#sidebar-shade")?.classList.toggle("open", isOpen);
  } else {
    state.sidebarCollapsed = layout.classList.toggle("sidebar-collapsed");
  }
  updateToggleLabel();
}

// Event delegation tetap bekerja setelah dashboard dirender ulang.
export function initSidebarControls() {
  document.addEventListener("click", (event) => {
    if (!(event.target instanceof Element)) return;

    if (event.target.id === "sidebar-shade") {
      closeMobileSidebar();
      return;
    }

    const control = event.target.closest("[data-action]");
    if (!control) return;

    if (control.dataset.action === "menu") toggleSidebar();
    if (["nav-dashboard", "nav-products"].includes(control.dataset.action))
      activateLink(control);
  });

  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && isMobileLayout()) closeMobileSidebar();
  });

  window.addEventListener("resize", updateToggleLabel);
}
