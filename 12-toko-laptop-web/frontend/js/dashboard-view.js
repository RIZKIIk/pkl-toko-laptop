// Tampilan dashboard toko, ringkasan stok, pencarian, dan daftar produk.
import { app, brand, esc, html, icons, roleName, state } from "./shared.js";
import { themeToggleMarkup } from "./theme.js";

function statCard(icon, label, value, tone) {
  return html`<article class="stat-card">
    <span class="stat-icon ${tone || ""}">${icon}</span>
    <div class="stat-copy"><span>${label}</span><strong>${value}</strong></div>
  </article>`;
}

export function normalizeLaptop(row) {
  return Array.isArray(row)
    ? { id: row[0], nama: row[1], merek: row[2], harga: row[3], stok: row[4] }
    : row;
}

// Membentuk daftar produk dan menerapkan pencarian lokal.
export function inventoryMarkup(laptops) {
  const query = state.search.trim().toLocaleLowerCase("id-ID");
  const matching = laptops.filter(
    (item) =>
      !query ||
      `${item.nama} ${item.merek} ${item.harga}`
        .toLocaleLowerCase("id-ID")
        .includes(query),
  );
  const visible = matching
    .filter((item) => {
      const stock = Number(item.stok || 0);
      if (state.stockFilter === "available") return stock > 0;
      if (state.stockFilter === "low") return stock > 0 && stock < 4;
      if (state.stockFilter === "out") return stock === 0;
      return true;
    })
    .sort((a, b) => {
      if (state.productSort === "name")
        return a.nama.localeCompare(b.nama, "id-ID");
      if (state.productSort === "price-asc")
        return (
          Number(String(a.harga).replace(/[^\d]/g, "")) -
          Number(String(b.harga).replace(/[^\d]/g, ""))
        );
      if (state.productSort === "price-desc")
        return (
          Number(String(b.harga).replace(/[^\d]/g, "")) -
          Number(String(a.harga).replace(/[^\d]/g, ""))
        );
      return 0;
    });
  if (!visible.length && !laptops.length) {
    const canCreate = state.user.role === "admin";
    return html`<div class="empty-state">
      <div class="empty-illustration">${icons.box}</div>
      <strong role="status" aria-live="polite">Belum ada produk laptop</strong>
      <p>Tambahkan laptop untuk mulai menjual di LaptopKu.</p>
      ${canCreate
        ? html`<button class="primary-button" data-action="create">
            ${icons.plus} Tambah laptop
          </button>`
        : ""}
    </div>`;
  }
  if (!visible.length)
    return html`<div class="empty-state">
      <div class="empty-illustration">${icons.search}</div>
      <strong role="status" aria-live="polite">Produk tidak ditemukan</strong>
      <p>Coba kata kunci lain atau tampilkan semua produk.</p>
      <button class="secondary-button" type="button" data-action="clear-filters">
        Hapus pencarian dan filter
      </button>
    </div>`;
  return html`<div class="table-scroll">
      <table class="inventory-table">
        <thead>
          <tr>
            <th>Produk</th>
            <th>Harga</th>
            <th>Stok</th>
            <th>ID produk</th>
            <th aria-label="Aksi"></th>
          </tr>
        </thead>
        <tbody id="inventory-body">
          ${visible
            .map((item) => {
              const stock = Number(item.stok || 0);
              const stockClass =
                stock === 0 ? "out" : stock < 4 ? "low" : "";
              const stockLabel = stock === 0 ? "Habis" : `${stock} unit`;
              return html`<tr>
                <td>
                  <div class="product-name">
                    <span class="product-thumb">${icons.laptop}</span>
                    <div>
                      <div class="product-title">${esc(item.nama)}</div>
                      <div class="product-subtitle">${esc(item.merek)}</div>
                    </div>
                  </div>
                </td>
                <td class="price-cell">${esc(item.harga)}</td>
                <td>
                  <span class="stock-pill ${stockClass}">${stockLabel}</span>
                </td>
                <td>#${esc(item.id)}</td>
                <td>
                  <button
                    class="row-action"
                    data-action="detail"
                    data-id="${esc(item.id)}"
                    aria-label="Lihat ${esc(item.nama)}"
                    title="Lihat detail"
                  >
                    ${icons.eye}
                  </button>
                </td>
              </tr>`;
            })
            .join("")}
        </tbody>
      </table>
    </div>
    <div class="table-footer">
      <span role="status" aria-live="polite">Menampilkan ${visible.length} dari ${laptops.length} produk</span
      ><span>Produk LaptopKu</span>
    </div>`;
}

// Menampilkan ringkasan dan daftar laptop sesuai role pengguna.
export function renderDashboard() {
  const user = state.user;
  const totalStock = state.laptops.reduce(
    (sum, item) => sum + Number(item.stok || 0),
    0,
  );
  const lowStock = state.laptops.filter(
    (item) => Number(item.stok) > 0 && Number(item.stok) < 4,
  ).length;
  const canCreate = user.role === "admin";
  const dateText = new Intl.DateTimeFormat("id-ID", {
    weekday: "short",
    day: "numeric",
    month: "short",
    year: "numeric",
  }).format(new Date());
  const initial = (user.username || "U").trim().charAt(0).toUpperCase();
  const isMobile = window.matchMedia("(max-width: 620px)").matches;
  const toggleLabel = isMobile
    ? "Buka menu"
    : state.sidebarCollapsed
      ? "Besarkan sidebar"
      : "Kecilkan sidebar";
  const toggleExpanded = String(isMobile ? false : !state.sidebarCollapsed);
  document.title = "Dashboard · LaptopKu";
  app.innerHTML = html` <div class="sidebar-shade" id="sidebar-shade"></div>
    <div class="app-layout${state.sidebarCollapsed ? " sidebar-collapsed" : ""}">
      <aside class="sidebar" id="sidebar">
        ${brand()}
        <p class="sidebar-label">Menu utama</p>
        <nav class="sidebar-nav" aria-label="Menu utama">
          <a
            class="nav-link active"
            href="#dashboard-top"
            data-action="nav-dashboard"
            aria-label="Dashboard"
            aria-current="page"
            title="Dashboard"
          >
            <span class="nav-icon">${icons.grid}</span>
            <span class="nav-label">Dashboard</span>
          </a>
          <a
            class="nav-link"
            href="#products"
            data-action="nav-products"
            aria-label="Produk laptop"
            title="Produk laptop"
          >
            <span class="nav-icon">${icons.laptop}</span>
            <span class="nav-label">Produk laptop</span>
          </a>
        </nav>
        <div class="sidebar-bottom">
          <div class="help-card">
            <strong>Butuh bantuan?</strong
            ><span>Hubungi admin toko.</span>
          </div>
          <div class="profile-mini">
            <span class="avatar">${esc(initial)}</span>
            <div class="profile-copy">
              <strong>${esc(user.username)}</strong
              ><span>${esc(roleName(user.role))}</span>
            </div>
            <button
              class="logout-button"
              data-action="logout"
              aria-label="Keluar"
              title="Keluar"
            >
              ${icons.logout}
            </button>
          </div>
        </div>
      </aside>
      <section class="main-area">
        <header class="topbar">
          <button
            class="sidebar-toggle"
            type="button"
            data-action="menu"
            aria-label="${toggleLabel}"
            aria-expanded="${toggleExpanded}"
            title="${toggleLabel}"
          >
            ${icons.menu}
          </button>
          <div class="breadcrumb">
            <span>LaptopKu</span><span aria-hidden="true">/</span
            ><strong>Dashboard</strong>
          </div>
          <div class="topbar-right">
            ${themeToggleMarkup()}
            <span class="date-chip">${esc(dateText)}</span
            ><span class="avatar top-avatar">${esc(initial)}</span>
          </div>
        </header>
        <main class="content" id="dashboard-top">
          <div class="welcome-row">
            <div>
              <p class="eyebrow eyebrow-compact">Ringkasan toko</p>
              <h1>Selamat datang, ${esc(user.username)}</h1>
              <p>Ringkasan produk dan stok LaptopKu.</p>
            </div>
            <div class="welcome-actions">
              <button class="secondary-button" data-action="refresh">
                ${icons.trend} Muat ulang</button
              >${canCreate
                ? html`<button class="primary-button" data-action="create">
                    ${icons.plus} Tambah laptop
                  </button>`
                : ""}
            </div>
          </div>
          <section class="stats-grid" aria-label="Ringkasan toko">
            ${statCard(icons.laptop, "Model laptop", state.laptops.length, "")}
            ${statCard(icons.box, "Total stok", totalStock, "green")}
            ${statCard(icons.trend, "Stok menipis", lowStock, "blue")}
          </section>
          <section class="inventory-panel" id="products">
            <div class="panel-heading">
              <div>
                <h2>Produk laptop</h2>
                <p>Laptop yang dijual di LaptopKu.</p>
              </div>
              <div class="panel-tools">
                <label class="search-box"
                  >${icons.search}<input
                    id="search"
                    type="search"
                    value="${esc(state.search)}"
                    placeholder="Cari nama atau merek..."
                    aria-label="Cari nama atau merek" /></label
                ><select
                  class="tool-select"
                  id="stock-filter"
                  aria-label="Filter berdasarkan stok"
                >
                  <option value="all" ${state.stockFilter === "all" ? "selected" : ""}>Semua stok</option>
                  <option value="available" ${state.stockFilter === "available" ? "selected" : ""}>Tersedia</option>
                  <option value="low" ${state.stockFilter === "low" ? "selected" : ""}>Menipis</option>
                  <option value="out" ${state.stockFilter === "out" ? "selected" : ""}>Habis</option>
                </select><select
                  class="tool-select sort-select"
                  id="product-sort"
                  aria-label="Urutkan produk"
                >
                  <option value="default" ${state.productSort === "default" ? "selected" : ""}>Urutan awal</option>
                  <option value="name" ${state.productSort === "name" ? "selected" : ""}>Nama A–Z</option>
                  <option value="price-asc" ${state.productSort === "price-asc" ? "selected" : ""}>Harga terendah</option>
                  <option value="price-desc" ${state.productSort === "price-desc" ? "selected" : ""}>Harga tertinggi</option>
                </select><span class="role-badge ${esc(user.role)}"
                  >${esc(roleName(user.role))}</span
                >
              </div>
            </div>
            <div id="inventory-content">${inventoryMarkup(state.laptops)}</div>
          </section>
        </main>
      </section>
    </div>`;
}
