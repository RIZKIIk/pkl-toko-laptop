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
          <p class="eyebrow">Toko LaptopKu</p>
          <h1>Laptop untuk<br /><span>kerja dan belajar.</span></h1>
          <p>Lihat pilihan laptop, harga, dan stok yang tersedia.</p>
          <div class="login-art" aria-hidden="true">
            <span class="art-spark spark-one">✳</span
            ><span class="art-spark spark-two">✦</span>
            <div class="laptop-art"></div>
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
        <p class="eyebrow">Daftar</p>
        <h2>Buat akun LaptopKu</h2>
        <p class="form-intro">Akun baru dapat digunakan untuk membeli laptop.</p>
        <div id="register-error" class="login-error" role="alert"></div>
        <form id="register-form" novalidate>
          <div class="field">
            <label for="register-username">Username</label
            ><input
              id="register-username"
              name="username"
              autocomplete="username"
              placeholder="Buat username"
              required
              maxlength="100"
            />
          </div>
          <div class="field">
            <label for="register-password">Password</label
            ><input
              id="register-password"
              name="password"
              type="password"
              autocomplete="new-password"
              placeholder="Buat password"
              required
            />
          </div>
          <div class="field">
            <label for="register-confirm-password">Ulangi password</label
            ><input
              id="register-confirm-password"
              name="confirmPassword"
              type="password"
              autocomplete="new-password"
              placeholder="Masukkan password sekali lagi"
              required
            />
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
