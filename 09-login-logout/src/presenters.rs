use crate::models::Laptop;
use serde::Serialize;
use serde_json::{Value, json};

#[derive(Debug, Serialize)]
pub struct LaptopResponse {
    pub id: i64,
    pub nama: String,
    pub merek: String,
    pub harga: String,
    pub stok: i32,
}

impl From<Laptop> for LaptopResponse {
    fn from(laptop: Laptop) -> Self {
        Self {
            id: laptop.id,
            nama: laptop.nama,
            merek: laptop.merek,
            harga: format_rupiah(laptop.harga),
            stok: laptop.stok,
        }
    }
}

impl LaptopResponse {
    pub fn to_array(&self) -> Value {
        json!([self.id, self.nama, self.merek, self.harga, self.stok])
    }
}

fn format_rupiah(harga: i64) -> String {
    let digits = harga.unsigned_abs().to_string();
    let mut reversed = String::new();
    for (index, digit) in digits.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            reversed.push('.');
        }
        reversed.push(digit);
    }
    let grouped: String = reversed.chars().rev().collect();
    let sign = if harga < 0 { "-" } else { "" };
    format!("Rp{sign}{grouped},00")
}
