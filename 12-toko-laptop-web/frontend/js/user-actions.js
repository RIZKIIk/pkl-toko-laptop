// Form admin untuk membuat akun dengan role yang disediakan backend.
import { api, state, toast } from "./shared.js";
import { renderCreateUserPage } from "./dashboard-view.js";

export async function openCreateUserPage() {
  if (state.user?.role !== "admin") {
    toast("Hanya admin yang dapat membuat akun pengguna.", true);
    location.replace("/dashboard");
    return;
  }

  try {
    const result = await api("/api/admin/roles");
    const roles = result.data?.roles || [];
    if (!roles.length) {
      renderCreateUserPage([], "Belum ada role yang tersedia untuk akun baru.");
      return;
    }

    renderCreateUserPage(roles);
    bindCreateUserForm(roles);
  } catch (error) {
    renderCreateUserPage([], error.message || "Daftar role belum dapat dimuat.");
  }
}

function bindCreateUserForm(roles) {
  const form = document.querySelector("#admin-user-form");
  if (!form) return;

  const roleSelect = form.elements.role;
  const roleDescription = document.querySelector("#admin-role-description");
  const message = document.querySelector("#admin-user-message");
  const submit = document.querySelector("#admin-user-submit");

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
      toast(
        `${result.message || "Akun berhasil dibuat"}: ${username} (${selectedRole?.label || role}).`,
      );
      form.reset();
      roleDescription.textContent =
        roles[0].description || "Role menentukan akses akun di toko.";
      form.elements.username.focus({ preventScroll: true });
    } catch (error) {
      message.textContent = error.message || "Akun belum dapat dibuat.";
      message.classList.add("visible");
    } finally {
      submit.disabled = false;
      submit.textContent = "Buat akun";
    }
  });
}
