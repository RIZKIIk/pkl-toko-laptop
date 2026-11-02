use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Laptop {
    pub id: i64,
    pub nama: String,
    pub merek: String,
    pub harga: i64,
    pub stok: i32,
}

#[derive(Debug, Deserialize)]
pub struct LaptopInput {
    pub nama: String,
    pub merek: String,
    pub harga: i64,
    pub stok: i32,
}

impl LaptopInput {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.nama.trim().is_empty() || self.merek.trim().is_empty() {
            return Err("Nama dan merek wajib diisi.");
        }
        if self.harga < 0 || self.stok < 0 {
            return Err("Harga dan stok tidak boleh bernilai negatif.");
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}
