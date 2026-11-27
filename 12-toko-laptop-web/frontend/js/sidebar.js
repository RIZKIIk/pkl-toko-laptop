// Interaksi sidebar: menu aktif, drawer ponsel, dan mode ringkas desktop.
import { state } from "./shared.js";

const MOBILE_LAYOUT = "(max-width: 620px)";

function isMobileLayout() {
  return window.matchMedia(MOBILE_LAYOUT).matches;
}

function updateToggleLabel() {
  const button = document.querySelector('[data-action="menu"]');
  const sidebar = document.querySelector("#sidebar");
  const brandButton = document.querySelector('[data-action="sidebar-toggle"]');
  if (!button || !sidebar) return;

  const mobile = isMobileLayout();
  const wasOpenOnMobile = !mobile && sidebar.classList.contains("open");
  if (!mobile) {
    sidebar.classList.remove("open");
    document.querySelector("#sidebar-shade")?.classList.remove("open");
    document.querySelector(".main-area")?.removeAttribute("inert");
    document.body.classList.remove("sidebar-open");
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
  const brandLabel = mobile
    ? expanded
      ? "Tutup menu"
      : "Buka menu"
    : expanded
      ? "Kecilkan sidebar"
      : "Besarkan sidebar";
  brandButton?.setAttribute("aria-expanded", String(expanded));
  brandButton?.setAttribute("aria-label", brandLabel);
  if (brandButton) brandButton.title = brandLabel;
  const visibleToggleLabel = button.querySelector(".menu-toggle-label");
  if (visibleToggleLabel)
    visibleToggleLabel.textContent = expanded ? "Tutup" : "Menu";
  sidebar.toggleAttribute("inert", mobile && !expanded);
  sidebar.setAttribute("aria-hidden", String(mobile && !expanded));
  const mainArea = document.querySelector(".main-area");
  if (mainArea) mainArea.toggleAttribute("inert", mobile && expanded);
  if (wasOpenOnMobile) button.focus({ preventScroll: true });
}

function closeMobileSidebar() {
  const wasOpen = document.querySelector("#sidebar")?.classList.contains("open");
  document.querySelector("#sidebar")?.classList.remove("open");
  document.querySelector("#sidebar-shade")?.classList.remove("open");
  document.querySelector(".main-area")?.removeAttribute("inert");
  document.body.classList.remove("sidebar-open");
  updateToggleLabel();
  if (wasOpen)
    document
      .querySelector('[data-action="menu"]')
      ?.focus({ preventScroll: true });
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
    document.querySelector(".main-area")?.toggleAttribute("inert", isOpen);
    document.body.classList.toggle("sidebar-open", isOpen);
    updateToggleLabel();
    document
      .querySelector(isOpen ? ".sidebar-dismiss" : '[data-action="menu"]')
      ?.focus({ preventScroll: true });
  } else {
    state.sidebarCollapsed = layout.classList.toggle("sidebar-collapsed");
    updateToggleLabel();
  }
}

// Event delegation tetap bekerja setelah dashboard dirender ulang.
export function initSidebarControls() {
  document.addEventListener("click", (event) => {
    if (!(event.target instanceof Element)) return;

    if (event.target.id === "sidebar-shade") {
      closeMobileSidebar();
      return;
    }

    const clickedSidebar = event.target.closest("#sidebar");
    if (
      clickedSidebar &&
      !isMobileLayout() &&
      !event.target.closest("a, button")
    ) {
      toggleSidebar();
      return;
    }

    const control = event.target.closest("[data-action]");
    if (!control) return;

    if (
      control.dataset.action === "menu" ||
      control.dataset.action === "sidebar-toggle"
    )
      toggleSidebar();
    if (control.dataset.action === "menu-close") closeMobileSidebar();
    if (
      ["nav-dashboard", "nav-products", "nav-create-user", "nav-profile"].includes(
        control.dataset.action,
      )
    )
      activateLink(control);
  });

  document.addEventListener("keydown", (event) => {
    const sidebar = document.querySelector("#sidebar");
    if (!sidebar?.classList.contains("open") || !isMobileLayout()) return;

    if (event.key === "Escape") {
      closeMobileSidebar();
      return;
    }

    if (event.key !== "Tab") return;
    const focusable = Array.from(
      sidebar.querySelectorAll('a[href], button:not([disabled])'),
    );
    if (!focusable.length) return;
    const first = focusable[0];
    const last = focusable.at(-1);
    if (event.shiftKey && document.activeElement === first) {
      event.preventDefault();
      last.focus();
    } else if (!event.shiftKey && document.activeElement === last) {
      event.preventDefault();
      first.focus();
    }
  });

  window.addEventListener("resize", updateToggleLabel);
}
