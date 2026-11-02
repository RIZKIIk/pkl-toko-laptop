//! Query PostgreSQL untuk membaca, membuat, mengubah, dan menghapus laptop.
//! Dokumentasi: file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app-lengkap/catatan/rust/repositories/laptop-repository.md
use crate::models::{CreateLaptopPayload, Laptop, UpdateLaptopPayload};
use sqlx::PgPool;

/// Mengambil semua laptop dengan urutan ID menaik.
pub async fn list_laptops(pool: &PgPool) -> Result<Vec<Laptop>, sqlx::Error> {
    sqlx::query_as::<_, Laptop>("SELECT id, nama, merek, harga, stok FROM laptops ORDER BY id")
        .fetch_all(pool)
        .await
}

/// Mengambil satu laptop; `None` berarti ID tidak ditemukan.
pub async fn get_laptop(pool: &PgPool, id: i64) -> Result<Option<Laptop>, sqlx::Error> {
    sqlx::query_as::<_, Laptop>("SELECT id, nama, merek, harga, stok FROM laptops WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Menyimpan laptop baru dan mengembalikan baris yang dibuat.
pub async fn create_laptop(
    pool: &PgPool,
    payload: &CreateLaptopPayload,
) -> Result<Laptop, sqlx::Error> {
    sqlx::query_as::<_, Laptop>(
        "INSERT INTO laptops (nama, merek, harga, stok)
         VALUES ($1, $2, $3, $4)
         RETURNING id, nama, merek, harga, stok",
    )
    .bind(&payload.nama)
    .bind(&payload.merek)
    .bind(payload.harga)
    .bind(payload.stok)
    .fetch_one(pool)
    .await
}

/// Mengubah field yang dikirim dan mengembalikan data terbaru bila ditemukan.
pub async fn update_laptop(
    pool: &PgPool,
    id: i64,
    payload: &UpdateLaptopPayload,
) -> Result<Option<Laptop>, sqlx::Error> {
    sqlx::query_as::<_, Laptop>(
        "UPDATE laptops
            SET nama = COALESCE($1, nama),
               merek = COALESCE($2, merek),
               harga = COALESCE($3, harga),
               stok = COALESCE($4, stok),
               updated_at = CURRENT_TIMESTAMP
            WHERE id = $5
            RETURNING id, nama, merek, harga, stok",
    )
    .bind(payload.nama.as_deref())
    .bind(payload.merek.as_deref())
    .bind(payload.harga)
    .bind(payload.stok)
    .bind(id)
    .fetch_optional(pool)
    .await
}

/// Menghapus laptop dan menyatakan apakah ada baris yang terhapus.
pub async fn delete_laptop(pool: &PgPool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM laptops WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
