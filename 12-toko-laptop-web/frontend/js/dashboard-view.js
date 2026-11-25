// Tampilan dashboard, ringkasan stok, pencarian, dan tabel inventaris.
import { app, brand, esc, html, icons, roleName, state } from "./shared.js";

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

// Membentuk tabel inventaris dan menerapkan pencarian lokal.
export function inventoryMarkup(laptops) {
  const query = state.search.trim().toLocaleLowerCase("id-ID");
  const visible = laptops.filter(
    (item) =>
      !query ||
      `${item.nama} ${item.merek} ${item.harga}`
        .toLocaleLowerCase("id-ID")
        .includes(query),
  );
  if (!visible.length && !laptops.length) {
    const canCreate = ["admin"].includes(state.user.role);
    return html`<div class="empty-state">
      <div class="empty-illustration">${icons.box}</div>
      <strong>Belum ada laptop di inventaris</strong>
      <p>Tambahkan produk agar katalog toko mulai terisi.</p>
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
      <strong>Produk tidak ditemukan</strong>
      <p>Coba kata kunci atau merek lain.</p>
    </div>`;
  return html`<div class="table-scroll">
      <table class="inventory-table">
        <thead>
          <tr>
            <th>Produk</th>
            <th>Harga</th>
            <th>Stok tersedia</th>
            <th>ID produk</th>
            <th aria-label="Aksi"></th>
          </tr>
        </thead>
        <tbody id="inventory-body">
          ${visible
            .map((item) => {
              const stockClass =
                item.stok === 0 ? "out" : item.stok < 4 ? "low" : "";
              const stockLabel =
                item.stok === 0 ? "Habis" : `${item.stok} unit`;
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
      <span>Menampilkan ${visible.length} dari ${laptops.length} produk</span
      ><span>Data inventaris terkini</span>
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
  document.title = "Dashboard · Ruang Laptop";
  app.innerHTML = html` <div class="sidebar-shade" id="sidebar-shade"></div>
    <div class="app-layout">
      <aside class="sidebar" id="sidebar">
        ${brand()}
        <p class="sidebar-label">Menu utama</p>
        <div class="nav-link active">
          ${`<span class="nav-icon">${icons.grid}</span>`}<span>Dashboard</span>
        </div>
        <div class="nav-link">
          ${`<span class="nav-icon">${icons.laptop}</span>`}<span
            >Inventaris laptop</span
          >
        </div>
        <div class="sidebar-bottom">
          <div class="help-card">
            <strong>Butuh bantuan?</strong
            ><span
              >Hubungi administrator jika Anda memerlukan akses atau bantuan
              akun.</span
            >
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
          <button class="mobile-menu" data-action="menu" aria-label="Buka menu">
            ${icons.menu}
          </button>
          <div class="breadcrumb">
            <span>Ruang Laptop</span><span aria-hidden="true">/</span
            ><strong>Dashboard</strong>
          </div>
          <div class="topbar-right">
            <span class="date-chip">${esc(dateText)}</span
            ><span class="avatar top-avatar">${esc(initial)}</span>
          </div>
        </header>
        <main class="content">
          <div class="welcome-row">
            <div>
              <p class="eyebrow eyebrow-compact">Ringkasan toko</p>
              <h1>Halo, ${esc(user.username)} 👋</h1>
              <p>Berikut kondisi inventaris laptop Anda hari ini.</p>
            </div>
            <div class="welcome-actions">
              <button class="secondary-button" data-action="refresh">
                ${icons.trend} Perbarui data</button
              >${canCreate
                ? html`<button class="primary-button" data-action="create">
                    ${icons.plus} Tambah laptop
                  </button>`
                : ""}
            </div>
          </div>
          <section class="stats-grid" aria-label="Ringkasan inventaris">
            ${statCard(icons.laptop, "Jenis laptop", state.laptops.length, "")}
            ${statCard(icons.box, "Total unit tersedia", totalStock, "green")}
            ${statCard(icons.trend, "Stok menipis", lowStock, "blue")}
          </section>
          <section class="inventory-panel">
            <div class="panel-heading">
              <div>
                <h2>Inventaris laptop</h2>
                <p>Daftar produk yang tersedia di toko Anda</p>
              </div>
              <div class="panel-tools">
                <label class="search-box"
                  >${icons.search}<input
                    id="search"
                    type="search"
                    value="${esc(state.search)}"
                    placeholder="Cari nama atau merek..."
                    aria-label="Cari nama atau merek" /></label
                ><span class="role-badge ${esc(user.role)}"
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
