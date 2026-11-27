# `src/handlers/laptops/mod.rs`

> **Navigasi:** [Indeks catatan](../../../README.md) · [README proyek](../../../../README.md) · [Endpoint API](../../../../endpoin.md)

![Ilustrasi: Modul laptop mengelompokkan aksi](../../../ilustrasi/halaman/20-mod.svg "Modul laptop mengelompokkan aksi")


[Buka source `laptops/mod.rs`](../../../../src/handlers/laptops/mod.rs)

## Tujuan

Modul ini mengelompokkan handler resource laptop, mengekspor fungsi untuk dipakai router, dan menyediakan helper response bersama. Modul ini tidak membaca request sendiri dan tidak menjalankan SQL.

## Pembagian file

| File | Endpoint/tugas | Hasil utama |
| --- | --- | --- |
| [`create.rs`](create.md) | `POST /api/laptops` | Membuat laptop dan mengembalikan object. |
| [`read.rs`](read.md) | `GET /api/laptops` dan `GET /api/laptops/{id}` | Mengembalikan daftar atau satu array laptop. |
| `buy.rs` | `POST /api/laptops/{id}/buy` | Membeli unit, mencatat actor UPDATE stok ke audit log. |
| [`update.rs`](update.md) | `PATCH /api/laptops/{id}` | Mengembalikan array laptop terbaru. |
| [`delete.rs`](delete.md) | `DELETE /api/laptops/{id}` | Mengembalikan pesan dan ID; trigger menyimpan snapshot DELETE. |

Setiap file operasi tetap menjadi modul internal (`mod create;`, dan seterusnya). Dengan cara itu, detail organisasi file terpisah dari jalur publik yang dipakai router.

## Ekspor handler

`pub use create::tambah_laptop;` mengekspor ulang satu fungsi dari modul anak. Baris yang sejenis mengekspor handler delete, read, dan update. Router kemudian dapat menggunakan jalur pendek seperti `handlers::laptops::tambah_laptop` tanpa mengimpor file implementasi satu per satu.

`pub use read::{ambil_laptop, daftar_laptop};` memakai kurung kurawal untuk memilih dua nama dari modul `read`. Re-export hanya membuat jalur nama yang lebih ringkas; ia tidak menyalin atau mengubah fungsi.

## Helper response

| Helper | Bentuk JSON | Dipakai untuk |
| --- | --- | --- |
| `respons_berhasil(data)` | `{"status":"success","data":...}` | GET daftar dan detail. |
| `respons_berhasil_dengan_pesan(message, data)` | Menambahkan `message` pada envelope sukses. | Create dan update. |
| `laptop_tidak_ditemukan(id)` | `ApiError::NotFound` dengan ID pada pesan. | Detail, update, dan delete. |

Helper menggunakan `serde_json::Value` agar dapat menerima array maupun object: `data` dapat berupa array atau object, sementara `message` adalah `&str`. Pada source, ketiga helper dideklarasikan `pub fn`, sehingga dapat dipanggil melalui jalur modul publik `handlers::laptops`. `pub` mengatur visibilitas item; ia tidak berarti fungsi otomatis menjadi endpoint HTTP.

### Signature helper

| Signature | Arti |
| --- | --- |
| `respons_berhasil(data: Value) -> Value` | Mengambil nilai JSON yang dimiliki fungsi dan membungkusnya dengan `status`. |
| `respons_berhasil_dengan_pesan(message: &str, data: Value) -> Value` | Meminjam pesan teks, mengambil data JSON, lalu membuat envelope sukses dengan pesan. |
| `laptop_tidak_ditemukan(id: i64) -> ApiError` | Membentuk error aplikasi `NotFound` untuk ID tertentu. |

Ketiga fungsi sinkron (`fn`, bukan `async fn`), karena hanya menyusun nilai di memori dan tidak menunggu I/O. `json!` adalah macro yang membuat nilai JSON. `laptop_tidak_ditemukan` memakai `format!` untuk menyisipkan ID ke dalam pesan.

## Alur pemanggilan

```text
endpoin/laptops.rs → handler operasi → helper bersama (jika dibutuhkan) → JSON/ApiError
```

Handler tetap menentukan urutan validasi dan query. Hasil query opsional biasanya dipetakan menjadi `Option<Laptop>`; bila tidak ada, handler memakai `laptop_tidak_ditemukan` untuk menghasilkan `ApiError::NotFound`. Helper di file ini hanya menyamakan bentuk response dan pesan not-found. Untuk alur tiap method, buka halaman [handlers](../../handlers.md) atau halaman operasi di tabel atas.
