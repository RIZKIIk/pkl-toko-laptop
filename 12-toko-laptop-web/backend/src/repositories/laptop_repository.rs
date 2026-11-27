//! Query PostgreSQL untuk membaca, membuat, mengubah, dan menghapus laptop.
//! Catatan: [Buka penjelasan](../../catatan/rust/repositories/laptop-repository.md)
use crate::models::{CreateLaptopPayload, Laptop, PurchaseReceipt, UpdateLaptopPayload};
use sqlx::{PgPool, Postgres, Transaction};

/// Menyimpan ID pengguna tepercaya untuk trigger log selama transaksi aktif.
async fn set_audit_actor(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query_scalar::<_, String>("SELECT set_config('rizzshop.user_id', $1, true)")
        .bind(user_id.to_string())
        .fetch_one(&mut **transaction)
        .await?;

    Ok(())
}

/// Mengambil semua laptop dengan urutan ID menaik.
pub async fn daftar_laptop(pool: &PgPool) -> Result<Vec<Laptop>, sqlx::Error> {
    sqlx::query_as::<_, Laptop>(
        "SELECT id, nama, merek, harga, stok, deskripsi FROM laptops ORDER BY id",
    )
    .fetch_all(pool)
    .await
}

/// Mengambil satu laptop; `None` berarti ID tidak ditemukan.
pub async fn ambil_laptop(pool: &PgPool, id: i64) -> Result<Option<Laptop>, sqlx::Error> {
    sqlx::query_as::<_, Laptop>(
        "SELECT id, nama, merek, harga, stok, deskripsi FROM laptops WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

/// Mengurangi stok hanya jika jumlah yang diminta masih tersedia.
pub async fn beli_laptop(
    pool: &PgPool,
    user_id: i64,
    username: &str,
    id: i64,
    jumlah: i32,
    payment_method: &str,
) -> Result<Option<(Laptop, PurchaseReceipt)>, sqlx::Error> {
    let mut transaction = pool.begin().await?;
    set_audit_actor(&mut transaction, user_id).await?;

    let laptop = sqlx::query_as::<_, Laptop>(
        "SELECT id, nama, merek, harga, stok, deskripsi
         FROM laptops WHERE id = $1 FOR UPDATE",
    )
    .bind(id)
    .fetch_optional(&mut *transaction)
    .await?;

    let Some(laptop) = laptop else {
        return Ok(None);
    };
    let Some(total_price) = laptop.harga.checked_mul(i64::from(jumlah)) else {
        return Ok(None);
    };
    if laptop.stok < jumlah {
        return Ok(None);
    }

    let updated_laptop = sqlx::query_as::<_, Laptop>(
        "UPDATE laptops
         SET stok = stok - $2, updated_at = CURRENT_TIMESTAMP
         WHERE id = $1
         RETURNING id, nama, merek, harga, stok, deskripsi",
    )
    .bind(id)
    .bind(jumlah)
    .fetch_one(&mut *transaction)
    .await?;

    let receipt = sqlx::query_as::<_, PurchaseReceipt>(
        "INSERT INTO purchases (
             user_id, username, laptop_id, laptop_name, laptop_brand,
             unit_price, quantity, total_price, payment_method, payment_status
         )
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'simulated_paid')
         RETURNING id, username, laptop_name, laptop_brand, unit_price,
                   quantity, total_price, payment_method, payment_status,
                   EXTRACT(EPOCH FROM created_at)::BIGINT AS created_at_epoch",
    )
    .bind(user_id)
    .bind(username)
    .bind(id)
    .bind(&laptop.nama)
    .bind(&laptop.merek)
    .bind(laptop.harga)
    .bind(jumlah)
    .bind(total_price)
    .bind(payment_method)
    .fetch_one(&mut *transaction)
    .await?;

    transaction.commit().await?;
    Ok(Some((updated_laptop, receipt)))
}

/// Menyimpan laptop baru dan mengembalikan baris yang dibuat.
pub async fn tambah_laptop(
    pool: &PgPool,
    user_id: i64,
    payload: &CreateLaptopPayload,
) -> Result<Laptop, sqlx::Error> {
    let mut transaction = pool.begin().await?;
    set_audit_actor(&mut transaction, user_id).await?;

    let laptop = sqlx::query_as::<_, Laptop>(
        "INSERT INTO laptops (nama, merek, harga, stok, deskripsi)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, nama, merek, harga, stok, deskripsi",
    )
    .bind(&payload.nama)
    .bind(&payload.merek)
    .bind(payload.harga)
    .bind(payload.stok)
    .bind(&payload.deskripsi)
    .fetch_one(&mut *transaction)
    .await?;

    transaction.commit().await?;
    Ok(laptop)
}

/// Mengubah field yang dikirim dan mengembalikan data terbaru bila ditemukan.
pub async fn ubah_laptop(
    pool: &PgPool,
    user_id: i64,
    id: i64,
    payload: &UpdateLaptopPayload,
) -> Result<Option<Laptop>, sqlx::Error> {
    let mut transaction = pool.begin().await?;
    set_audit_actor(&mut transaction, user_id).await?;

    // COALESCE mempertahankan nilai lama untuk field PATCH yang bernilai None.
    let laptop = sqlx::query_as::<_, Laptop>(
        "UPDATE laptops
            SET nama = COALESCE($1, nama),
               merek = COALESCE($2, merek),
               deskripsi = COALESCE($3, deskripsi),
               harga = COALESCE($4, harga),
               stok = COALESCE($5, stok),
               updated_at = CURRENT_TIMESTAMP
            WHERE id = $6
            RETURNING id, nama, merek, harga, stok, deskripsi",
    )
    .bind(payload.nama.as_deref())
    .bind(payload.merek.as_deref())
    .bind(payload.deskripsi.as_deref())
    .bind(payload.harga)
    .bind(payload.stok)
    .bind(id)
    .fetch_optional(&mut *transaction)
    .await?;

    transaction.commit().await?;
    Ok(laptop)
}

/// Menghapus laptop dan menyatakan apakah ada baris yang terhapus.
pub async fn hapus_laptop(pool: &PgPool, user_id: i64, id: i64) -> Result<bool, sqlx::Error> {
    let mut transaction = pool.begin().await?;
    set_audit_actor(&mut transaction, user_id).await?;

    let result = sqlx::query("DELETE FROM laptops WHERE id = $1")
        .bind(id)
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await?;
    Ok(result.rows_affected() > 0)
}
