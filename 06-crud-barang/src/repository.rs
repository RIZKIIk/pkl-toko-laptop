use crate::models::{Laptop, LaptopInput};
use sqlx::PgPool;

pub async fn list(pool: &PgPool) -> Result<Vec<Laptop>, sqlx::Error> {
    sqlx::query_as::<_, Laptop>(
        "SELECT id, nama, merek, harga, stok
         FROM public.laptops
         ORDER BY id",
    )
    .fetch_all(pool)
    .await
}

pub async fn find(pool: &PgPool, id: i64) -> Result<Option<Laptop>, sqlx::Error> {
    sqlx::query_as::<_, Laptop>(
        "SELECT id, nama, merek, harga, stok FROM public.laptops WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create(pool: &PgPool, input: &LaptopInput) -> Result<Laptop, sqlx::Error> {
    sqlx::query_as::<_, Laptop>(
        "INSERT INTO public.laptops (nama, merek, harga, stok)
         VALUES ($1, $2, $3, $4)
         RETURNING id, nama, merek, harga, stok",
    )
    .bind(input.nama.trim())
    .bind(input.merek.trim())
    .bind(input.harga)
    .bind(input.stok)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: i64,
    input: &LaptopInput,
) -> Result<Option<Laptop>, sqlx::Error> {
    sqlx::query_as::<_, Laptop>(
        "UPDATE public.laptops
         SET nama = $1, merek = $2, harga = $3, stok = $4, updated_at = CURRENT_TIMESTAMP
         WHERE id = $5
         RETURNING id, nama, merek, harga, stok",
    )
    .bind(input.nama.trim())
    .bind(input.merek.trim())
    .bind(input.harga)
    .bind(input.stok)
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM public.laptops WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}
