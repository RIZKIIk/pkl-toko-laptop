// Halaman login dan pengiriman kredensial ke backend.
import { ApiError, api, app, brand, html, icons, saveToken } from "./shared.js";

// Menampilkan form login dan mengirim kredensial ke endpoint autentikasi.
export function renderLogin() {
  document.title = "Masuk · Ruang Laptop";
  const expired = new URLSearchParams(location.search).has("session");
  app.innerHTML = html` <section class="login-page">
    <div class="login-shell">
      <aside class="login-aside">
        ${brand()}
        <div class="login-promo">
          <p class="eyebrow">Pusat inventaris laptop</p>
          <h1>Semua stok,<br /><span>satu tempat.</span></h1>
          <p>
            Kelola katalog laptop, pantau persediaan, dan bantu pelanggan
            menemukan perangkat yang tepat.
          </p>
          <div class="login-art" aria-hidden="true">
            <span class="art-spark spark-one">✳</span
            ><span class="art-spark spark-two">✦</span>
            <div class="laptop-art"></div>
          </div>
        </div>
        <div class="login-foot">
          © ${new Date().getFullYear()} Ruang Laptop · Sistem inventaris
        </div>
      </aside>
      <section class="login-form-side">
        ${brand("mobile-brand")}
        <p class="eyebrow">Selamat datang kembali</p>
        <h2>Masuk ke akun</h2>
        <p class="form-intro">
          Masukkan username dan password untuk melanjutkan ke dashboard.
        </p>
        <div
          id="login-error"
          class="login-error${expired ? " visible" : ""}"
          role="alert"
        >
          Sesi berakhir. Silakan masuk kembali.
        </div>
        <form id="login-form" novalidate>
          <div class="field">
            <label for="username">Username</label
            ><input
              id="username"
              name="username"
              autocomplete="username"
              placeholder="Contoh: nama pengguna"
              required
              maxlength="100"
            />
          </div>
          <div class="field">
            <label for="password">Password</label>
            <div class="input-wrap">
              <input
                class="password-input"
                id="password"
                name="password"
                type="password"
                autocomplete="current-password"
                placeholder="Masukkan password"
                required
              /><button
                class="text-button password-toggle"
                id="password-toggle"
                type="button"
                aria-label="Tampilkan password"
              >
                Lihat
              </button>
            </div>
          </div>
          <button
            class="primary-button login-submit"
            id="login-submit"
            type="submit"
          >
            Masuk <span aria-hidden="true">→</span>
          </button>
        </form>
        <p class="login-help">
          Belum punya akun? Hubungi administrator untuk mendapatkan akses.
        </p>
        <p class="secure-note">
          ${icons.lock} Koneksi login dilindungi session aman
        </p>
      </section>
    </div>
  </section>`;

  const form = document.querySelector("#login-form");
  const errorBox = document.querySelector("#login-error");
  document
    .querySelector("#password-toggle")
    .addEventListener("click", (event) => {
      const input = document.querySelector("#password");
      const visible = input.type === "password";
      input.type = visible ? "text" : "password";
      event.currentTarget.textContent = visible ? "Sembunyi" : "Lihat";
      event.currentTarget.setAttribute(
        "aria-label",
        visible ? "Sembunyikan password" : "Tampilkan password",
      );
    });
  form.addEventListener("submit", async (event) => {
    event.preventDefault();
    errorBox.classList.remove("visible");
    const username = form.elements.username.value.trim();
    const password = form.elements.password.value;
    if (!username || !password) {
      errorBox.textContent = !username
        ? "Username wajib diisi."
        : "Password wajib diisi.";
      errorBox.classList.add("visible");
      (!username ? form.elements.username : form.elements.password).focus();
      return;
    }
    const button = document.querySelector("#login-submit");
    button.disabled = true;
    button.innerHTML = html`<span
        class="loader loader-small loader-light"
      ></span>
      Memeriksa akun...`;
    try {
      const result = await api("/api/auth/login", {
        method: "POST",
        auth: false,
        body: JSON.stringify({ username, password }),
      });
      const auth = result.data || {};
      if (!auth.access_token || !auth.username || !auth.role)
        throw new ApiError("Response login tidak lengkap.", 500);
      saveToken(auth.access_token);
      location.assign("/dashboard");
    } catch (error) {
      errorBox.textContent = error.message;
      errorBox.classList.add("visible");
      button.disabled = false;
      button.innerHTML = html`Masuk <span aria-hidden="true">→</span>`;
    }
  });
}
