# Gambaran Proyek

> **Navigasi:** [Indeks catatan](README.md) · [README proyek](../README.md) · [Endpoint API](../endpoin.md)

![Ilustrasi: Peta wilayah aplikasi](ilustrasi/halaman/02-01-gambaran-proyek.svg "Peta wilayah aplikasi")


Toko Laptop App adalah REST API inventaris laptop. Kode aktif menyediakan operasi CRUD, registrasi akun, role `admin`/`buyer`/`restocker`, serta login berbasis cookie session dan JWT bearer. Migration audit memasang trigger PostgreSQL yang mencatat INSERT, UPDATE, dan DELETE laptop beserta actor terautentikasi bila perubahan datang dari backend.

## Teknologi yang benar-benar dipakai

| Teknologi | Dipakai untuk | Contoh di kode |
| --- | --- | --- |
| Rust 2024 | Bahasa aplikasi. | `Cargo.toml`. |
| Axum | Router, handler, extractor, dan response HTTP. | `endpoin/`, `handlers/`, `error.rs`. |
| Tokio | Runtime async dan listener TCP. | `#[tokio::main]`, `TcpListener`. |
| SQLx | Pool dan query PostgreSQL. | `config.rs`, `repositories/`. |
| PostgreSQL | Menyimpan akun, inventaris, dan audit log. | Baseline SQLx dan migration audit lanjutan; log memakai FK `SET NULL` agar tetap ada setelah penghapusan laptop/pengguna. |
| Serde | Membaca JSON body ke payload Rust. | `#[derive(Deserialize)]`. |
| serde_json | Membuat JSON response. | `json!`, `Value`. |
| dotenvy | Mencoba memuat environment dari `.env`. | `main.rs`. |
| Postman | Mengirim request CRUD dan mengecek response. | folder `postman/`. |

## Struktur proyek

| Lokasi | Tanggung jawab |
| --- | --- |
| `src/main.rs` | Startup program: env, config, database, router, port, server. |
| `src/lib.rs` | Mendaftarkan semua modul aplikasi. |
| `src/config.rs` | Membaca environment dan membuat pool PostgreSQL. |
| `src/endpoin/` | Menghubungkan URL dan method ke handler per fitur. |
| `src/routes.rs` | Jalur kompatibilitas ke router di `endpoin/`. |
| `src/handlers/` | Handler dipisahkan menurut resource dan operasi HTTP. |
| `src/models/` | Entity database dan payload POST/PATCH dipisah per tipe. |
| `src/validation/` | Validasi ID, teks, create, dan update. |
| `src/repositories/` | Query SQL dikelompokkan menurut resource. |
| `src/presenters/` | Pemetaan response dan format Rupiah dipisah. |
| `src/error.rs` | Memetakan error menjadi response HTTP JSON. |
| `database/migrations/` | Riwayat perubahan skema akun, inventaris, dan audit log laptop. |
| `.env` | Pengaturan lokal server dan PostgreSQL; tidak dibagikan sebagai dokumentasi. |
| `postman/` | Collection serta environment untuk uji manual. |
| `catatan/` | Penjelasan detail kode; tidak dijalankan oleh aplikasi. |

## Pembagian tanggung jawab

Contoh `POST /api/laptops` melewati beberapa lapisan yang berbeda.

```text
JSON request
    ↓
handler menerima Json<CreateLaptopPayload>
    ↓
validation membersihkan dan memeriksa nilai
    ↓
repository menjalankan INSERT
    ↓
PostgreSQL mengembalikan baris baru
    ↓
presenter membuat JSON object dan format Rupiah
    ↓
handler mengirim 201 Created
```

Pemisahan folder dan file mengikuti tanggung jawab, bukan menambah lapisan bisnis baru. `Json<T>` dari Axum meminta body JSON dibaca menjadi tipe Rust `T` yang mendukung Serde `Deserialize`; `Json<Value>` mengubah nilai JSON yang disiapkan presenter menjadi body response. Jika JSON body tidak dapat diparse menjadi tipe yang diminta, extractor dapat mengirim error sebelum handler berjalan.

Setiap file tetap memiliki fokus kecil:

- Ingin tahu URL? Buka [routes](rust/routes.md).
- Ingin tahu alur request? Buka [handlers](rust/handlers.md) lalu catatan tiap [operasi laptop](rust/handlers/laptops/mod.md).
- Ingin tahu aturan input? Buka [validation](rust/validation.md).
- Ingin tahu SQL? Buka [repositories](rust/repositories.md).
- Ingin tahu bentuk response? Buka [presenters](rust/presenters.md).

## Istilah Rust yang muncul berulang

| Istilah | Arti dalam project ini |
| --- | --- |
| `mod` | Mendaftarkan bagian kode sebagai modul. |
| `use` | Membawa nama dari library/modul agar dapat ditulis pendek. |
| `pub` | Membuka struct/fungsi/modul agar dapat dipakai bagian lain. |
| `async` dan `.await` | Menandai dan menunggu operasi I/O seperti database dan jaringan. |
| `Result` | Nilai bisa sukses (`Ok`) atau gagal (`Err`). |
| `Option` | Nilai bisa ada (`Some`) atau tidak ada (`None`), dipakai untuk hasil query dan field PATCH. |
| `?` | Meneruskan error dan menghentikan fungsi saat langkah sebelumnya gagal. |
| `&T` | Meminjam nilai `T` tanpa memindahkan kepemilikannya. |
| `Result<T, E>` | Mengandung `Ok(T)` saat berhasil atau `Err(E)` saat gagal; `Result` berbeda dari `Option`, yang menyatakan ada/tidaknya nilai. |

Migration lama tetap menjadi baseline yang sama. Perubahan audit dan penghapusan kolom lama `token_version` dikelola oleh migration lanjutan; lihat [skema, relasi, dan trigger](rust/database/initial-schema.md) serta [DBML](../database/schema.dbml). Buat dan verifikasi backup sebelum menerapkan migration ke database lama.

Lanjutkan ke [alur kerja aplikasi](02-alur-kerja-aplikasi.md) untuk melihat istilah tersebut saat request benar-benar berjalan.
