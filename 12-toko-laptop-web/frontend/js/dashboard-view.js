// Tampilan dashboard toko, ringkasan stok, pencarian, dan daftar produk.
import { app, esc, html, icons, roleName, state } from "./shared.js";
import { themeToggleMarkup } from "./theme.js";

function statCard(icon, label, value, tone, note) {
  return html`<article class="stat-card">
    <span class="stat-icon ${tone || ""}">${icon}</span>
    <div class="stat-copy">
      <span>${label}</span><strong>${value}</strong><small>${note}</small>
    </div>
  </article>`;
}

export function normalizeLaptop(row) {
  return Array.isArray(row)
    ? {
        id: row[0],
        nama: row[1],
        merek: row[2],
        harga: row[3],
        stok: row[4],
        deskripsi: row[5] || "",
      }
    : row;
}

// Membentuk daftar produk dan menerapkan pencarian lokal.
export function inventoryMarkup(laptops) {
  const query = state.search.trim().toLocaleLowerCase("id-ID");
  const role = state.user?.role;
  const canBuy = role === "buyer" || role === "admin";
  const canEdit = role === "restocker" || role === "admin";
  const canDelete = role === "admin";
  const matching = laptops.filter(
    (item) =>
      !query ||
      `${item.nama} ${item.merek} ${item.harga} ${item.deskripsi || ""}`
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
            <th>Aksi</th>
          </tr>
        </thead>
        <tbody id="inventory-body">
          ${visible
            .map((item) => {
              const stock = Number(item.stok || 0);
              const stockClass =
                stock === 0 ? "out" : stock < 4 ? "low" : "";
              const stockLabel = stock === 0 ? "Habis" : `${stock} unit`;
              const productName = esc(item.nama);
              return html`<tr>
                <td>
                  <div class="product-name">
                    <span class="product-thumb">${icons.laptop}</span>
                    <div>
                      <div class="product-title">${productName}</div>
                      <div class="product-subtitle">${esc(item.merek)}</div>
                      <div class="product-description-preview">${esc(item.deskripsi || "")}</div>
                    </div>
                  </div>
                </td>
                <td class="price-cell">${esc(item.harga)}</td>
                <td>
                  <span class="stock-pill ${stockClass}">${stockLabel}</span>
                </td>
                <td>#${esc(item.id)}</td>
                <td class="inventory-actions-cell">
                  <div class="inventory-actions" role="group" aria-label="Aksi ${productName}">
                    <button
                      class="row-action row-action-detail"
                      data-action="detail"
                      data-id="${esc(item.id)}"
                      aria-label="Lihat detail ${productName}"
                      title="Lihat detail"
                    >${icons.eye}<span class="row-action-label">Detail</span></button>
                    ${canEdit
                      ? html`<button
                          class="row-action row-action-edit"
                          data-action="edit"
                          data-id="${esc(item.id)}"
                          aria-label="Ubah data ${productName}"
                          title="Ubah data"
                        >${icons.pencil}<span class="row-action-label">Ubah</span></button>`
                      : ""}
                    ${canBuy
                      ? html`<button
                          class="row-action row-action-buy"
                          data-action="buy"
                          data-id="${esc(item.id)}"
                          aria-label="Beli ${productName}"
                          title="Beli laptop"
                          ${stock === 0 ? "disabled" : ""}
                        >${icons.cart}<span class="row-action-label">Beli</span></button>`
                      : ""}
                    ${canDelete
                      ? html`<button
                          class="row-action row-action-delete"
                          data-action="delete"
                          data-id="${esc(item.id)}"
                          aria-label="Hapus ${productName}"
                          title="Hapus laptop"
                        >${icons.trash}<span class="row-action-label">Hapus</span></button>`
                      : ""}
                  </div>
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

function renderAppShell(pageTitle, page, content) {
  const user = state.user;
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
  app.innerHTML = html` <div class="sidebar-shade" id="sidebar-shade"></div>
    <div class="app-layout${state.sidebarCollapsed ? " sidebar-collapsed" : ""}">
      <aside
        class="sidebar"
        id="sidebar"
        aria-label="Navigasi utama"
        aria-hidden="${isMobile}"
        ${isMobile ? "inert" : ""}
      >
        <div class="sidebar-header">
          <button
            class="sidebar-brand-toggle"
            type="button"
            data-action="sidebar-toggle"
            aria-controls="sidebar"
            aria-expanded="${toggleExpanded}"
            aria-label="${toggleLabel}"
            title="${toggleLabel}"
          >
            <span class="brand">
              <span class="brand-mark">${icons.laptop}</span>
              <span class="brand-name">Laptop<span class="brand-accent">Ku</span></span>
            </span>
          </button><button
            class="sidebar-dismiss"
            type="button"
            data-action="menu-close"
            aria-label="Tutup menu navigasi"
            title="Tutup menu"
          >
            ${icons.close}
          </button>
        </div>
        <p class="sidebar-label">Menu utama</p>
        <nav class="sidebar-nav" aria-label="Menu utama">
          <a
            class="nav-link${page === "dashboard" ? " active" : ""}"
            href="${page === "dashboard" || page === "products" ? "#dashboard-top" : "/dashboard"}"
            data-action="nav-dashboard"
            aria-label="Dashboard"
            ${page === "dashboard" ? 'aria-current="page"' : ""}
            title="Dashboard"
          >
            <span class="nav-icon">${icons.grid}</span>
            <span class="nav-label">Dashboard</span>
          </a>
          <a
            class="nav-link${page === "products" ? " active" : ""}"
            href="${page === "dashboard" || page === "products" ? "#products" : "/dashboard#products"}"
            data-action="nav-products"
            aria-label="Produk laptop"
            ${page === "products" ? 'aria-current="page"' : ""}
            title="Produk laptop"
          >
            <span class="nav-icon">${icons.laptop}</span>
            <span class="nav-label">Produk laptop</span>
          </a>
          <a
            class="nav-link${page === "profile" ? " active" : ""}"
            href="/dashboard/profile"
            data-action="nav-profile"
            aria-label="Profil"
            ${page === "profile" ? 'aria-current="page"' : ""}
            title="Profil"
          >
            <span class="nav-icon">${icons.profile}</span>
            <span class="nav-label">Profil</span>
          </a>
          ${user.role === "admin"
            ? html`<a
                class="nav-link${page === "users" ? " active" : ""}"
                href="/dashboard/users/new"
                data-action="nav-create-user"
                aria-label="Tambah akun"
                ${page === "users" ? 'aria-current="page"' : ""}
                title="Tambah akun"
              >
                <span class="nav-icon">${icons.plus}</span>
                <span class="nav-label">Tambah akun</span>
              </a>`
            : ""}
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
          </div>
        </div>
      </aside>
      <section class="main-area">
        <header class="topbar">
          <button
            class="sidebar-toggle"
            type="button"
            data-action="menu"
            aria-controls="sidebar"
            aria-label="${toggleLabel}"
            aria-expanded="${toggleExpanded}"
            title="${toggleLabel}"
          >
            <span class="menu-toggle-label">Menu</span>
          </button>
          <div class="breadcrumb">
            <span>LaptopKu</span><span aria-hidden="true">/</span
            ><strong>${esc(pageTitle)}</strong>
          </div>
          <div class="topbar-right">
            ${themeToggleMarkup()}
            <div class="account-menu">
              <button
                class="account-trigger"
                type="button"
                data-action="profile-menu-toggle"
                aria-expanded="false"
                aria-controls="account-menu"
                aria-label="Profil ${esc(user.username)}"
                title="Profil ${esc(user.username)}"
              >
                <span class="avatar top-avatar">${esc(initial)}</span>
                <span class="account-trigger-copy">
                  <strong>${esc(user.username)}</strong>
                  <small>${esc(roleName(user.role))}</small>
                </span>
                <span class="account-trigger-chevron">${icons.chevronDown}</span>
              </button>
              <div class="account-popover" id="account-menu" aria-label="Menu akun" hidden>
                <div class="account-popover-summary">
                  <span class="avatar account-popover-avatar">${esc(initial)}</span>
                  <span class="account-popover-copy">
                    <strong>${esc(user.username)}</strong>
                    <small>${esc(roleName(user.role))}</small>
                  </span>
                </div>
                <span class="account-popover-id">ID akun · #${esc(user.id)}</span>
                <a class="account-menu-link" href="/dashboard/profile">
                  ${icons.profile}<span>Lihat profil</span>
                </a>
                <button class="account-menu-logout" type="button" data-action="logout">
                  ${icons.logout}<span>Keluar dari akun</span>
                </button>
              </div>
            </div>
            <span class="date-chip">${esc(dateText)}</span>
          </div>
        </header>
        <main class="content" id="dashboard-top">${content}</main>
      </section>
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
  document.title = "Dashboard · LaptopKu";
  const content = html`<section class="dashboard-hero" aria-labelledby="dashboard-heading">
          <div class="dashboard-hero-copy">
            <div class="dashboard-hero-meta">
              <span class="dashboard-kicker"><span class="dashboard-kicker-mark" aria-hidden="true"></span> Ringkasan toko</span>
              <span class="role-badge ${esc(user.role)}">${esc(roleName(user.role))}</span>
            </div>
            <h1 id="dashboard-heading">Selamat datang, ${esc(user.username)}</h1>
            <p>Pantau katalog dan kondisi stok laptop toko dalam satu tampilan.</p>
          </div>
          <div class="dashboard-hero-actions">
            <a class="hero-link-button" href="#products">${icons.laptop} Lihat produk</a>
            <button class="hero-refresh-button" type="button" data-action="refresh">
              ${icons.trend} Muat ulang
            </button>
            ${canCreate
              ? html`<button class="hero-primary-button" type="button" data-action="create">
                  ${icons.plus} Tambah laptop
                </button>`
              : ""}
          </div>
        </section>
          <section class="stats-grid" aria-label="Ringkasan toko">
            ${statCard(icons.laptop, "Model laptop", state.laptops.length, "", "Jenis produk terdaftar")}
            ${statCard(icons.box, "Total stok", new Intl.NumberFormat("id-ID").format(totalStock), "green", "Unit di seluruh katalog")}
            ${statCard(icons.trend, "Stok menipis", lowStock, "blue", lowStock ? "Model perlu restok" : "Semua stok aman")}
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
                </select>
              </div>
            </div>
            <div id="inventory-content">${inventoryMarkup(state.laptops)}</div>
          </section>`;

  const activePage = location.hash === "#products" ? "products" : "dashboard";
  renderAppShell(
    activePage === "products" ? "Produk laptop" : "Dashboard",
    activePage,
    content,
  );
  if (location.hash === "#products") {
    requestAnimationFrame(() => {
      document.querySelector("#products")?.scrollIntoView({ block: "start" });
    });
  }
}

// Form pembuatan akun ditampilkan di halaman khusus dari menu admin.
export function renderCreateUserPage(roles, errorMessage = "") {
  document.title = "Tambah akun · LaptopKu";
  const content = html`<div class="welcome-row">
      <div>
        <p class="eyebrow eyebrow-compact">Pengelolaan pengguna</p>
        <h1>Tambah akun</h1>
        <p>Buat akun baru dan pilih akses yang sesuai untuk pengguna toko.</p>
      </div>
    </div>
    <section
      class="inventory-panel admin-user-panel"
      aria-labelledby="admin-user-title"
    >
      <div class="panel-heading">
        <div>
          <h2 id="admin-user-title">Informasi akun</h2>
          <p>Isi username, password awal, dan role akun.</p>
        </div>
      </div>
      <div class="admin-user-page-body">
        ${errorMessage
          ? html`<p class="dialog-message visible" role="alert">
                ${esc(errorMessage)}
              </p>
              <button
                class="secondary-button"
                type="button"
                data-action="retry-account"
              >
                Coba lagi
              </button>`
          : html`<form class="admin-user-form" id="admin-user-form">
              <div class="field">
                <label for="admin-username">Username</label>
                <input
                  id="admin-username"
                  name="username"
                  autocomplete="off"
                  maxlength="100"
                  placeholder="Contoh: petugas-toko"
                  required
                />
              </div>
              <div class="field">
                <label for="admin-password">Password awal</label>
                <input
                  id="admin-password"
                  name="password"
                  type="password"
                  autocomplete="new-password"
                  placeholder="Buat password untuk akun ini"
                  required
                />
              </div>
              <div class="field">
                <label for="admin-user-role">Role akun</label>
                <select id="admin-user-role" name="role" required>
                  ${roles
                    .map(
                      (role) =>
                        html`<option value="${esc(role.value)}">${esc(role.label)}</option>`,
                    )
                    .join("")}
                </select>
                <small
                  class="role-description"
                  id="admin-role-description"
                  >${esc(roles[0]?.description || "Role menentukan akses akun di toko.")}</small
                >
              </div>
              <p class="dialog-message" id="admin-user-message" role="alert" aria-live="polite"></p>
              <div class="admin-user-page-actions">
                <a class="secondary-button" href="/dashboard">Batal</a>
                <button class="primary-button" id="admin-user-submit" type="submit">Buat akun</button>
              </div>
            </form>`}
      </div>
    </section>`;

  renderAppShell("Tambah akun", "users", content);
}

// Menampilkan identitas akun dan aksesnya untuk semua role pengguna.
export function renderProfilePage() {
  const user = state.user;
  const username = user.username || "Pengguna";
  const initial = username.trim().charAt(0).toUpperCase() || "P";
  const accessDescription = {
    admin: "Mengelola produk, akun pengguna, dan aktivitas toko.",
    buyer: "Melihat katalog laptop dan melakukan simulasi pembelian.",
    restocker: "Melihat katalog laptop dan memperbarui stok produk.",
  }[user.role] || "Akses akun mengikuti role yang diberikan admin toko.";

  document.title = "Profil · LaptopKu";
  const content = html`<div class="welcome-row profile-welcome">
      <div>
        <p class="eyebrow eyebrow-compact">Akun LaptopKu</p>
        <h1>Profil saya</h1>
        <p>Informasi akun dan akses yang sedang digunakan.</p>
      </div>
    </div>
    <section class="profile-overview" aria-labelledby="profile-name">
      <div class="profile-overview-head">
        <span class="avatar profile-avatar">${esc(initial)}</span>
        <div class="profile-identity">
          <span class="profile-kicker">Nama pengguna</span>
          <h2 id="profile-name">${esc(username)}</h2>
          <span class="profile-identity-note">Akun LaptopKu</span>
        </div>
        <span class="role-badge ${esc(user.role)}">${esc(roleName(user.role))}</span>
      </div>
      <div class="profile-details-grid">
        <div class="profile-detail">
          <span>ID akun</span>
          <strong>#${esc(user.id)}</strong>
        </div>
        <div class="profile-detail">
          <span>Status sesi</span>
          <strong class="profile-session-status"><i aria-hidden="true"></i>Aktif</strong>
        </div>
        <div class="profile-detail profile-access-detail">
          <span>Akses akun</span>
          <strong>${esc(accessDescription)}</strong>
        </div>
      </div>
    </section>`;

  renderAppShell("Profil", "profile", content);
}
