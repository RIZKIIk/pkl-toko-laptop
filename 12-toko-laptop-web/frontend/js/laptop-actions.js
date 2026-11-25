// Dialog detail serta aksi tambah, ubah, hapus, dan pembelian laptop.
import {
  api,
  dialog,
  esc,
  html,
  icons,
  reportError,
  state,
  toast,
} from "./shared.js";
import { normalizeLaptop } from "./dashboard-view.js";
import { refreshDashboard } from "./dashboard.js";

function showDialog(title, subtitle, body, onReady) {
  dialog.innerHTML = html`<div class="dialog-head">
      <div>
        <h2>${title}</h2>
        <p>${subtitle}</p>
      </div>
      <button
        class="dialog-close"
        data-action="close-dialog"
        aria-label="Tutup"
      >
        ×
      </button>
    </div>
    <div class="dialog-body">${body}</div>`;
  dialog.showModal();
  if (onReady) onReady();
}

function laptopForm(laptop = {}) {
  return html`<form id="laptop-form">
    <div class="field">
      <label for="laptop-name">Nama laptop</label
      ><input
        id="laptop-name"
        name="nama"
        required
        maxlength="150"
        placeholder="Contoh: ThinkPad E14"
        value="${esc(laptop.nama || "")}"
      />
    </div>
    <div class="field">
      <label for="laptop-brand">Merek</label
      ><input
        id="laptop-brand"
        name="merek"
        required
        maxlength="100"
        placeholder="Contoh: Lenovo"
        value="${esc(laptop.merek || "")}"
      />
    </div>
    <div class="field">
      <label for="laptop-price">Harga (Rupiah)</label
      ><input
        id="laptop-price"
        name="harga"
        type="number"
        min="0"
        step="1"
        required
        placeholder="12500000"
        value="${laptop.hargaNumber ?? ""}"
      />
    </div>
    <div class="field">
      <label for="laptop-stock">Jumlah stok</label
      ><input
        id="laptop-stock"
        name="stok"
        type="number"
        min="0"
        step="1"
        required
        placeholder="5"
        value="${laptop.stok ?? ""}"
      />
    </div>
    <div class="dialog-actions">
      <button class="secondary-button" type="button" data-action="close-dialog">
        Batal</button
      ><button class="primary-button" type="submit">
        ${laptop.id ? "Simpan perubahan" : "Simpan laptop"}
      </button>
    </div>
  </form>`;
}

function parseRupiah(value) {
  return Number(String(value || "").replace(/[^\d]/g, ""));
}

// Form ini mengirim POST untuk laptop baru atau PATCH untuk perubahan.
function bindLaptopForm(laptop = null) {
  const form = document.querySelector("#laptop-form");
  form.addEventListener("submit", async (event) => {
    event.preventDefault();
    const submit = form.querySelector('[type="submit"]');
    submit.disabled = true;
    const payload = {
      nama: form.elements.nama.value.trim(),
      merek: form.elements.merek.value.trim(),
      harga: Number(form.elements.harga.value),
      stok: Number(form.elements.stok.value),
    };
    try {
      const result = await api(
        laptop ? `/api/laptops/${laptop.id}` : "/api/laptops",
        {
          method: laptop ? "PATCH" : "POST",
          body: JSON.stringify(payload),
        },
      );
      dialog.close();
      toast(
        result.message ||
          (laptop
            ? "Laptop berhasil diperbarui."
            : "Laptop berhasil ditambahkan."),
      );
      await refreshDashboard();
    } catch (error) {
      reportError(error);
      submit.disabled = false;
    }
  });
}

export function openCreate() {
  if (state.user.role !== "admin")
    return toast("Hanya admin yang dapat menambahkan laptop.", true);
  showDialog(
    "Tambah laptop",
    "Masukkan informasi laptop yang akan dijual.",
    laptopForm(),
    () => bindLaptopForm(),
  );
}

// Memuat detail dari API agar stok dan harga yang tampil selalu terbaru.
export async function openDetail(id) {
  try {
    const result = await api(`/api/laptops/${encodeURIComponent(id)}`);
    const laptop = normalizeLaptop(result.data);
    laptop.hargaNumber = parseRupiah(laptop.harga);
    const role = state.user.role;
    const canBuy = role === "buyer" || role === "admin";
    const canEdit = role === "restocker" || role === "admin";
    const canDelete = role === "admin";
    showDialog(
      esc(laptop.nama),
      esc(laptop.merek),
      html`
        <div class="detail-hero">${icons.laptop}</div>
        <div class="detail-list">
          <div class="detail-item">
            <span>Harga</span><strong>${esc(laptop.harga)}</strong>
          </div>
          <div class="detail-item">
            <span>Stok tersedia</span><strong>${laptop.stok} unit</strong>
          </div>
          <div class="detail-item">
            <span>ID produk</span><strong>#${esc(laptop.id)}</strong>
          </div>
          <div class="detail-item">
            <span>Merek</span><strong>${esc(laptop.merek)}</strong>
          </div>
        </div>
        ${canBuy && laptop.stok > 0
          ? html`<form id="buy-form">
              <div class="field">
                <label for="buy-quantity">Jumlah pembelian</label
                ><input
                  id="buy-quantity"
                  name="jumlah"
                  type="number"
                  min="1"
                  max="${Number(laptop.stok)}"
                  value="1"
                  required
                />
              </div>
              <div class="dialog-actions dialog-actions-spread">
                <button
                  class="secondary-button"
                  type="button"
                  data-action="close-dialog"
                >
                  Tutup</button
                ><button class="primary-button" type="submit">
                  Beli laptop
                </button>
              </div>
            </form>`
          : html`<div class="dialog-actions dialog-actions-spread">
              <button
                class="secondary-button"
                type="button"
                data-action="close-dialog"
              >
                Tutup</button
              >${canBuy
                ? html`<button class="primary-button" type="button" disabled>
                    Stok habis
                  </button>`
                : ""}
            </div>`}
        ${canEdit || canDelete
          ? html`<div class="dialog-actions dialog-actions-start">
              ${canEdit
                ? html`<button
                    class="secondary-button"
                    type="button"
                    data-action="edit"
                    data-id="${esc(laptop.id)}"
                  >
                    Ubah data
                  </button>`
                : ""}${canDelete
                ? html`<button
                    class="secondary-button button-danger"
                    type="button"
                    data-action="delete"
                    data-id="${esc(laptop.id)}"
                  >
                    Hapus produk
                  </button>`
                : ""}
            </div>`
          : ""}
      `,
      () => {
        const buyForm = document.querySelector("#buy-form");
        buyForm?.addEventListener("submit", async (event) => {
          event.preventDefault();
          const button = buyForm.querySelector('[type="submit"]');
          button.disabled = true;
          try {
            const result = await api(
              `/api/laptops/${encodeURIComponent(laptop.id)}/buy`,
              {
                method: "POST",
                body: JSON.stringify({
                  jumlah: Number(buyForm.elements.jumlah.value),
                }),
              },
            );
            dialog.close();
            toast(
              `Pembelian berhasil · ${result.data.jumlah_dibeli} unit ${result.data.laptop.nama}.`,
            );
            await refreshDashboard();
          } catch (error) {
            reportError(error);
            button.disabled = false;
          }
        });
      },
    );
  } catch (error) {
    reportError(error);
  }
}

export async function openEdit(id) {
  try {
    const result = await api(`/api/laptops/${encodeURIComponent(id)}`);
    const laptop = normalizeLaptop(result.data);
    laptop.hargaNumber = parseRupiah(laptop.harga);
    showDialog(
      "Ubah data laptop",
      "Perbarui informasi dan stok produk.",
      laptopForm(laptop),
      () => bindLaptopForm(laptop),
    );
  } catch (error) {
    reportError(error);
  }
}

export async function deleteLaptop(id) {
  const item = state.laptops.find((laptop) => String(laptop.id) === String(id));
  if (!window.confirm(`Hapus ${item?.nama || "laptop ini"} dari inventaris?`))
    return;
  try {
    const result = await api(`/api/laptops/${encodeURIComponent(id)}`, {
      method: "DELETE",
    });
    dialog.close();
    toast(result.message || "Laptop berhasil dihapus.");
    await refreshDashboard();
  } catch (error) {
    reportError(error);
  }
}
