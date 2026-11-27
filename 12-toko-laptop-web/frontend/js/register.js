// Halaman pendaftaran akun pembeli LaptopKu.
import { api, app, brand, html } from "./shared.js";
import { themeToggleMarkup } from "./theme.js";

export function renderRegister() {
  document.title = "Daftar · LaptopKu";
  app.innerHTML = html`<section class="login-page">
    <div class="login-shell">
      <aside class="login-aside">
        ${brand()}
        <div class="login-promo">
          <p class="eyebrow">Katalog LaptopKu</p>
          <h1>Mau cari laptop? Cari di Laptop<span>Ku</span>.</h1>
          <p>Daftar untuk melihat laptop yang tersedia di katalog.</p>
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
        <h2>Buat akun pembeli</h2>
        <p class="form-intro">
          Daftar untuk melihat katalog. Akun staf dibuat oleh admin toko.
        </p>
        <div id="register-error" class="login-error" role="alert"></div>
        <form id="register-form" novalidate>
          <div class="field">
            <label for="register-username">Username</label
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
                id="register-username"
                name="username"
                autocomplete="username"
                placeholder="Buat username"
                required
                maxlength="100"
              />
            </div>
          </div>
          <div class="field">
            <label for="register-password">Password</label
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
                  <rect x="4" y="10" width="16" height="11" rx="2" />
                  <path d="M8 10V7a4 4 0 1 1 8 0v3" />
                </svg>
              </span>
              <input
                class="icon-input"
                id="register-password"
                name="password"
                type="password"
                autocomplete="new-password"
                placeholder="Buat password"
                required
              />
            </div>
          </div>
          <div class="field">
            <label for="register-confirm-password">Ulangi password</label
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
                  <rect x="4" y="10" width="16" height="11" rx="2" />
                  <path d="M8 10V7a4 4 0 1 1 8 0v3" />
                </svg>
              </span>
              <input
                class="icon-input"
                id="register-confirm-password"
                name="confirmPassword"
                type="password"
                autocomplete="new-password"
                placeholder="Masukkan password sekali lagi"
                required
              />
            </div>
          </div>
          <button
            class="primary-button login-submit"
            id="register-submit"
            type="submit"
          >
            Buat akun
          </button>
        </form>
        <p class="login-help">
          Sudah punya akun? <a class="auth-link" href="/login">Masuk</a>
        </p>
      </section>
    </div>
  </section>`;

  const form = document.querySelector("#register-form");
  const message = document.querySelector("#register-error");
  const button = document.querySelector("#register-submit");

  form.addEventListener("submit", async (event) => {
    event.preventDefault();
    message.classList.remove("visible", "success");

    const username = form.elements.username.value.trim();
    const password = form.elements.password.value;
    const confirmPassword = form.elements.confirmPassword.value;
    if (!username || !password || !confirmPassword) {
      message.textContent = "Isi username dan kedua kolom password.";
      message.classList.add("visible");
      return;
    }
    if (password !== confirmPassword) {
      message.textContent = "Password yang dimasukkan belum sama.";
      message.classList.add("visible");
      form.elements.confirmPassword.focus();
      return;
    }

    button.disabled = true;
    button.textContent = "Membuat akun...";
    try {
      await api("/api/auth/register", {
        method: "POST",
        auth: false,
        body: JSON.stringify({ username, password }),
      });
      location.assign("/login?registered=1");
    } catch (error) {
      message.textContent = error.message;
      message.classList.add("visible");
      button.disabled = false;
      button.textContent = "Buat akun";
    }
  });
}
