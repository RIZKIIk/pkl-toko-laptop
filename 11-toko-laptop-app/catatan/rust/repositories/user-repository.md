# `src/repositories/user_repository.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

![Ilustrasi: Akses data akun](../../ilustrasi/halaman/40-user-repository.svg "Akses data akun")


[Buka source `user_repository.rs`](../../../src/repositories/user_repository.rs)

## Tujuan

File ini mengisolasi query PostgreSQL pada tabel `public.users`. Ia tidak memproses request HTTP atau membuat session; layanan di [`auth/mod.rs`](../auth/mod.md) memanggil fungsi repository ini.

> **Catatan keamanan:** kolom `password` dan perbandingan password di proyek ini menggunakan teks biasa untuk latihan session lokal. Jangan gunakan dengan akun nyata atau di produksi.

## Import

```rust
use sqlx::PgPool;
```

`PgPool` adalah pool koneksi SQLx ke PostgreSQL yang dibuat saat startup. Fungsi repository meminjamnya lewat `&PgPool`, sehingga koneksi dikelola bersama dan tidak dibuat ulang pada setiap query.

## 1. `find_password`

```rust
pub async fn find_password(
    pool: &PgPool,
    username: &str,
) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_scalar("SELECT password FROM public.users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
}
```

- `pub async fn` mendeklarasikan fungsi publik yang dapat menunggu operasi database.
- `username: &str` adalah username yang dipinjam dari pemanggil.
- `Result<Option<String>, sqlx::Error>` membedakan error query dari username tidak ditemukan:
  - `Ok(Some(password))`: baris ditemukan.
  - `Ok(None)`: tidak ada username tersebut.
  - `Err(error)`: query gagal.
- `query_scalar` mengambil satu nilai kolom, bukan seluruh baris sebagai struct.
- `$1` adalah placeholder parameter PostgreSQL. `.bind(username)` mengisi parameter itu secara aman.
- `fetch_optional(pool)` mengambil nol atau satu hasil.
- `.await` menunggu query selesai. Ekspresi terakhir langsung menjadi hasil fungsi.

## 2. `upsert_user`

```rust
pub async fn upsert_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO public.users (username, password) VALUES ($1, $2) \
         ON CONFLICT (username) DO UPDATE SET password = EXCLUDED.password",
    )
    .bind(username)
    .bind(password)
    .execute(pool)
    .await?;

    Ok(())
}
```

Fungsi ini memasukkan akun baru. Jika username sudah ada, `ON CONFLICT (username) DO UPDATE` mengganti password dengan nilai baru; `EXCLUDED.password` merujuk nilai password dari baris yang sedang dicoba untuk dimasukkan.

1. `sqlx::query(...)` membuat query yang tidak perlu membaca baris kembali.
2. `.bind(username)` dan `.bind(password)` memasangkan nilai ke `$1` dan `$2`. Jangan menyambung nilai user langsung ke SQL.
3. `.execute(pool).await?` menjalankan query. `?` meneruskan `sqlx::Error` jika operasi gagal.
4. `Ok(())` menyatakan berhasil tanpa mengembalikan baris.

## 3. `insert_user`

```rust
pub async fn insert_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<bool, sqlx::Error> {
    let inserted_username: Option<String> = sqlx::query_scalar(
        "INSERT INTO public.users (username, password) VALUES ($1, $2) \
         ON CONFLICT (username) DO NOTHING RETURNING username",
    )
    .bind(username)
    .bind(password)
    .fetch_optional(pool)
    .await?;

    Ok(inserted_username.is_some())
}
```

Berbeda dari `upsert_user`, fungsi ini tidak menimpa akun. Jika username konflik, `DO NOTHING` tidak menyisipkan apa pun.

- `let inserted_username: Option<String>` memberi nama dan tipe pada hasil: mungkin ada username hasil insert, mungkin `None`.
- `RETURNING username` meminta database mengembalikan username jika baris benar-benar dibuat.
- `fetch_optional` menghasilkan `Some(...)` untuk akun baru atau `None` jika konflik tidak membuat baris.
- `.is_some()` mengubah `Option` menjadi `bool`: `true` berarti akun dibuat, `false` berarti username sudah digunakan.
- `Ok(...)` menandai kedua hasil tersebut sebagai operasi database yang berhasil. Kegagalan query tetap menjadi `Err(sqlx::Error)`.

## Perbedaan fungsi

| Fungsi | Username baru | Username sudah ada |
| --- | --- | --- |
| `upsert_user` | Membuat akun | Memperbarui password |
| `insert_user` | Membuat akun dan menghasilkan `true` | Tidak mengubah data dan menghasilkan `false` |

Skema tabel dijelaskan pada [migration users](../database/0002-create-users.md) dan [migration password latihan](../database/0003-plaintext-password.md).

## Ilustrasi operasi akun

```text
login    → find_password → Some(password) / None
startup  → upsert_user   → insert atau perbarui akun admin
register → insert_user   → true (baru) / false (username dipakai)
```
