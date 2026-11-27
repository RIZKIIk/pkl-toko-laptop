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
  const wasOpen = dialog.open;
  dialog.classList.remove("receipt-dialog");
  dialog.innerHTML = html`<div class="dialog-head">
      <div>
        <h2 id="dialog-title">${title}</h2>
        <p>${subtitle}</p>
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
    <div class="dialog-body">${body}</div>`;
  dialog.setAttribute("aria-labelledby", "dialog-title");
  if (!wasOpen) dialog.showModal();
  if (onReady) onReady();
  dialog
    .querySelector("#laptop-form input, #buy-form input, .dialog-close")
    ?.focus({ preventScroll: true });
}

function laptopForm(laptop = {}) {
  return html`<form id="laptop-form">
    <div class="form-grid">
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
    </div>
    <div class="field field-full">
      <label for="laptop-description">Deskripsi laptop</label>
      <textarea
        id="laptop-description"
        name="deskripsi"
        required
        maxlength="1000"
        rows="4"
        placeholder="Jelaskan spesifikasi utama dan cocok untuk kebutuhan apa."
      >${esc(laptop.deskripsi || "")}</textarea>
      <small class="field-hint">Maksimal 1.000 karakter. Deskripsi tampil di katalog.</small>
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

function rupiah(value) {
  return new Intl.NumberFormat("id-ID", {
    style: "currency",
    currency: "IDR",
    maximumFractionDigits: 0,
  }).format(Number(value) || 0);
}

const paymentLabels = {
  qris: "QRIS (simulasi)",
  cash: "Tunai (simulasi)",
  bank_transfer: "Transfer bank (simulasi)",
  debit_card: "Kartu debit (simulasi)",
};

function receiptMarkup(receipt) {
  const date = new Date(Number(receipt.dibuat_pada) * 1000);
  const dateLabel = new Intl.DateTimeFormat("id-ID", {
    dateStyle: "long",
    timeStyle: "short",
    timeZone: "Asia/Jakarta",
  }).format(date);
  return html`<section class="receipt" aria-label="Struk pembelian">
    <div class="receipt-brand"><span class="receipt-brand-mark">LK</span><span>LaptopKu</span></div>
    <div class="receipt-status"><span class="receipt-status-dot"></span> LUNAS · SIMULASI</div>
    <div class="receipt-heading"><span>Struk pembelian</span><strong>${esc(receipt.nomor)}</strong></div>
    <div class="receipt-meta">
      <div><span>Tanggal</span><strong>${esc(dateLabel)} WIB</strong></div>
      <div><span>Pembeli</span><strong>${esc(receipt.username)}</strong></div>
    </div>
    <div class="receipt-divider"></div>
    <div class="receipt-product">
      <strong>${esc(receipt.nama_laptop)}</strong>
      <span>${esc(receipt.merek_laptop)}</span>
      <div><span>${receipt.jumlah} unit × ${rupiah(receipt.harga_satuan)}</span><strong>${rupiah(receipt.total)}</strong></div>
    </div>
    <div class="receipt-total"><span>Total dibayar</span><strong>${rupiah(receipt.total)}</strong></div>
    <div class="receipt-meta receipt-payment">
      <div><span>Metode</span><strong>${esc(paymentLabels[receipt.metode_pembayaran] || receipt.metode_pembayaran)}</strong></div>
      <div><span>Status</span><strong>Berhasil (simulasi)</strong></div>
    </div>
    <p class="receipt-note">Transaksi ini hanya simulasi untuk keperluan demonstrasi. Tidak ada uang yang ditagihkan.</p>
    <div class="dialog-actions receipt-actions no-print">
      <button class="secondary-button" type="button" data-action="close-dialog">Tutup</button>
      <button class="primary-button" type="button" data-action="print-receipt">Cetak struk</button>
    </div>
  </section>`;
}

function showReceipt(receipt) {
  showDialog("Pembelian berhasil", "Simpan atau cetak bukti transaksi simulasi.", receiptMarkup(receipt));
  dialog.classList.add("receipt-dialog");
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
      deskripsi: form.elements.deskripsi.value.trim(),
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
    showDialog(
      esc(laptop.nama),
      esc(laptop.merek),
      html`
        <div class="detail-hero">${icons.laptop}</div>
        <section class="detail-description">
          <span>Tentang laptop ini</span>
          <p>${esc(laptop.deskripsi || "Deskripsi belum tersedia.")}</p>
        </section>
        <div class="detail-list">
          <div class="detail-item">
            <span>Harga</span><strong>${esc(laptop.harga)}</strong>
          </div>
          <div class="detail-item">
            <span>Stok</span><strong>${laptop.stok} unit</strong>
          </div>
          <div class="detail-item">
            <span>ID produk</span><strong>#${esc(laptop.id)}</strong>
          </div>
          <div class="detail-item">
            <span>Merek</span><strong>${esc(laptop.merek)}</strong>
          </div>
        </div>
        <div class="dialog-actions dialog-actions-spread">
          <span class="detail-stock-note${Number(laptop.stok) > 0 ? " in-stock" : " out-of-stock"}">
            ${Number(laptop.stok) > 0 ? `Tersedia ${laptop.stok} unit` : "Stok habis"}
          </span>
          <button class="secondary-button" type="button" data-action="close-dialog">
            Tutup
          </button>
        </div>
      `,
    );
  } catch (error) {
    reportError(error);
  }
}

function bindBuyForm(laptop) {
  const buyForm = document.querySelector("#buy-form");
  const quantityInput = buyForm?.elements.jumlah;
  const paymentSelect = buyForm?.elements.metode_pembayaran;
  const updateCheckoutSummary = () => {
    const quantity = Math.max(1, Number(quantityInput?.value) || 1);
    const quantityLabel = document.querySelector("#checkout-quantity");
    const totalLabel = document.querySelector("#checkout-total");
    if (quantityLabel) quantityLabel.textContent = `${quantity} unit`;
    if (totalLabel) totalLabel.textContent = rupiah(laptop.hargaNumber * quantity);
    const paymentMessage = document.querySelector("#payment-simulation-message");
    if (paymentMessage) {
      const method = paymentLabels[paymentSelect?.value] || "Metode pembayaran";
      paymentMessage.textContent = `${method} dipilih. Tekan tombol untuk menyelesaikan simulasi; tidak ada uang yang ditagihkan.`;
    }
  };
  quantityInput?.addEventListener("input", updateCheckoutSummary);
  paymentSelect?.addEventListener("change", updateCheckoutSummary);
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
            metode_pembayaran: buyForm.elements.metode_pembayaran.value,
          }),
        },
      );
      showReceipt(result.data.struk);
      await refreshDashboard();
    } catch (error) {
      reportError(error);
      button.disabled = false;
    }
  });
}

export async function openBuy(id) {
  if (!state.user || !["buyer", "admin"].includes(state.user.role))
    return toast("Akun ini tidak memiliki akses untuk membeli laptop.", true);
  try {
    const result = await api(`/api/laptops/${encodeURIComponent(id)}`);
    const laptop = normalizeLaptop(result.data);
    if (Number(laptop.stok) < 1)
      return toast("Stok laptop sedang habis.", true);
    laptop.hargaNumber = parseRupiah(laptop.harga);
    showDialog(
      "Beli laptop",
      `${laptop.nama} · ${laptop.merek}`,
      html`<div class="checkout-product">
          <span class="checkout-product-icon">${icons.laptop}</span>
          <div><strong>${esc(laptop.nama)}</strong><span>${esc(laptop.merek)} · ${Number(laptop.stok)} unit tersedia</span></div>
          <strong class="checkout-product-price">${esc(laptop.harga)}</strong>
        </div>
        <p class="checkout-description">${esc(laptop.deskripsi || "")}</p>
        <form id="buy-form">
          <div class="checkout-fields">
            <div class="field">
              <label for="buy-quantity">Jumlah pembelian</label>
              <input id="buy-quantity" name="jumlah" type="number" min="1" max="${Number(laptop.stok)}" value="1" required />
            </div>
            <div class="field payment-field">
              <label for="payment-method">Metode pembayaran</label>
              <select id="payment-method" name="metode_pembayaran" required>
                <option value="qris">QRIS (simulasi)</option>
                <option value="cash">Tunai</option>
                <option value="bank_transfer">Transfer bank</option>
                <option value="debit_card">Kartu debit</option>
              </select>
            </div>
          </div>
          <div class="checkout-summary" aria-live="polite">
            <div><span>Harga satuan</span><strong>${esc(laptop.harga)}</strong></div>
            <div><span>Jumlah</span><strong id="checkout-quantity">1 unit</strong></div>
            <div class="checkout-total"><span>Total simulasi</span><strong id="checkout-total">${esc(laptop.harga)}</strong></div>
          </div>
          <p class="simulation-note" id="payment-simulation-message" role="status" aria-live="polite">QRIS dipilih. Tidak ada QR sungguhan atau tagihan ke rekening.</p>
          <div class="dialog-actions dialog-actions-spread">
            <button class="secondary-button" type="button" data-action="close-dialog">Batal</button>
            <button class="primary-button" type="submit">${icons.cart} Bayar &amp; buat struk</button>
          </div>
        </form>`,
      () => bindBuyForm(laptop),
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
  if (!window.confirm(`Hapus ${item?.nama || "laptop ini"} dari toko?`))
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
