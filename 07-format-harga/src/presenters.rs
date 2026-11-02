use crate::models::Laptop;
use serde::Serialize;

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

pub fn format_rupiah(harga: i64) -> String {
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

#[cfg(test)]
mod tests {
    use super::format_rupiah;

    #[test]
    fn formats_rupiah_with_thousands_separators() {
        assert_eq!(format_rupiah(15_000_000), "Rp15.000.000,00");
    }

    #[test]
    fn handles_negative_values_and_i64_minimum() {
        assert_eq!(format_rupiah(-1_500), "Rp-1.500,00");
        assert_eq!(format_rupiah(i64::MIN), "Rp-9.223.372.036.854.775.808,00");
    }
}
