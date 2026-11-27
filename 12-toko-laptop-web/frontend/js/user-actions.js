// Form admin untuk membuat akun dengan role yang disediakan backend.
import { api, dialog, esc, html, icons, state, toast } from "./shared.js";

export async function openCreateUser() {
  if (state.user?.role !== "admin") {
    toast("Hanya admin yang dapat membuat akun pengguna.", true);
    return;
  }

  let roles;
  try {
    const result = await api("/api/admin/roles");
    roles = result.data?.roles || [];
  } catch (error) {
    toast(error.message || "Daftar role belum dapat dimuat.", true);
    return;
  }

  if (!roles.length) {
    toast("Belum ada role yang tersedia untuk akun baru.", true);
    return;
  }

  dialog.innerHTML = html`<div class="dialog-head">
      <div>
        <h2 id="dialog-title">Buat akun pengguna</h2>
        <p>Pilih role sesuai tugas pengguna di toko.</p>
      </div>
      <button
        class="dialog-close"
        data-action="close-dialog"
        type="button"
        aria-label="Tutup"
        title="Tutup dialog"
      >
        ${icons.close}
      </button>
    </div>
    <div class="dialog-body">
      <form class="admin-user-form" id="admin-user-form">
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
          <small class="role-description" id="admin-role-description">
            ${esc(roles[0].description || "Role menentukan akses akun di toko.")}
          </small>
        </div>
        <p
          class="dialog-message"
          id="admin-user-message"
          role="alert"
          aria-live="polite"
        ></p>
        <div class="dialog-actions">
          <button
            class="secondary-button"
            type="button"
            data-action="close-dialog"
          >
            Batal
          </button>
          <button class="primary-button" id="admin-user-submit" type="submit">
            Buat akun
          </button>
        </div>
      </form>
    </div>`;

  dialog.setAttribute("aria-labelledby", "dialog-title");
  if (!dialog.open) dialog.showModal();

  const form = dialog.querySelector("#admin-user-form");
  const roleSelect = form.elements.role;
  const roleDescription = dialog.querySelector("#admin-role-description");
  const message = dialog.querySelector("#admin-user-message");
  const submit = dialog.querySelector("#admin-user-submit");

  form.elements.username.focus({ preventScroll: true });
  roleSelect.addEventListener("change", () => {
    const selectedRole = roles.find((role) => role.value === roleSelect.value);
    roleDescription.textContent =
      selectedRole?.description || "Role menentukan akses akun di toko.";
  });

  form.addEventListener("submit", async (event) => {
    event.preventDefault();
    message.classList.remove("visible");
    submit.disabled = true;
    submit.textContent = "Membuat akun...";

    const username = form.elements.username.value.trim();
    const password = form.elements.password.value;
    const role = roleSelect.value;
    if (!username) {
      message.textContent = "Username wajib diisi.";
      message.classList.add("visible");
      form.elements.username.focus();
      submit.disabled = false;
      submit.textContent = "Buat akun";
      return;
    }
    try {
      const result = await api("/api/admin/users", {
        method: "POST",
        body: JSON.stringify({ username, password, role }),
      });
      const selectedRole = roles.find((item) => item.value === role);
      dialog.close();
      toast(
        `${result.message || "Akun berhasil dibuat"}: ${username} (${selectedRole?.label || role}).`,
      );
    } catch (error) {
      message.textContent = error.message || "Akun belum dapat dibuat.";
      message.classList.add("visible");
      submit.disabled = false;
      submit.textContent = "Buat akun";
    }
  });
}
