// Halaman login dan pengiriman kredensial ke backend.
import { ApiError, api, app, brand, html, icons, saveToken } from "./shared.js";
import { themeToggleMarkup } from "./theme.js";

// Menampilkan form login dan mengirim kredensial ke endpoint autentikasi.
export function renderLogin() {
  document.title = "Masuk · LaptopKu";
  const query = new URLSearchParams(location.search);
  const expired = query.has("session");
  const registered = query.has("registered");
  app.innerHTML = html` <section class="login-page">
    <div class="login-shell">
      <aside class="login-aside">
        ${brand()}
        <div class="login-promo">
          <p class="eyebrow">Katalog LaptopKu</p>
          <h1>Mau cari laptop? Cari di Laptop<span>Ku</span>.</h1>
          <p>Masuk untuk melihat laptop yang tersedia di katalog.</p>
          <div class="login-art" aria-hidden="true">
            <img
              class="login-illustration"
              src="/assets/images/laptopku-hero.svg"
              width="560"
              height="300"
              alt=""
            />
          </div>
        </div>
        <div class="login-foot">
          © ${new Date().getFullYear()} LaptopKu · Toko laptop
        </div>
      </aside>
      <section class="login-form-side">
        <div class="auth-top">
          ${brand("mobile-brand")}${themeToggleMarkup()}
        </div>
        <p class="eyebrow">Akun LaptopKu</p>
        <h2>Masuk ke akun</h2>
        <p class="form-intro">Gunakan username dan password akunmu.</p>
        <div
          id="login-error"
          class="login-error${expired || registered ? " visible" : ""}${registered ? " success" : ""}"
          role="alert"
        >
          ${expired
            ? "Sesi berakhir. Silakan masuk kembali."
            : "Akun berhasil dibuat. Silakan masuk."}
        </div>
        <form id="login-form" novalidate>
          <div class="field">
            <label for="username">Username</label
            ><div class="input-wrap input-with-icon">
              <span class="input-icon" aria-hidden="true">
                <svg
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="1.8"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                >
                  <circle cx="12" cy="8" r="4" />
                  <path d="M5 21v-2a7 7 0 0 1 14 0v2" />
                </svg>
              </span>
              <input
                class="icon-input"
                id="username"
                name="username"
                autocomplete="username"
                placeholder="Masukkan username"
                required
                maxlength="100"
              />
            </div>
          </div>
          <div class="field">
            <label for="password">Password</label>
            <div class="input-wrap input-with-icon">
              <span class="input-icon" aria-hidden="true">
                <svg
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="1.8"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                >
                  <rect x="4" y="10" width="16" height="11" rx="2" />
                  <path d="M8 10V7a4 4 0 1 1 8 0v3" />
                </svg>
              </span>
              <input
                class="password-input icon-input"
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
                aria-controls="password"
                aria-pressed="false"
                title="Tampilkan password"
              >
                ${icons.eye}
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
          Belum punya akun?
          <a class="auth-link" href="/register">Daftar di sini</a>
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
      event.currentTarget.innerHTML = visible ? icons.eyeOff : icons.eye;
      event.currentTarget.setAttribute(
        "aria-label",
        visible ? "Sembunyikan password" : "Tampilkan password",
      );
      event.currentTarget.setAttribute("aria-pressed", String(visible));
      event.currentTarget.title = visible
        ? "Sembunyikan password"
        : "Tampilkan password";
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
