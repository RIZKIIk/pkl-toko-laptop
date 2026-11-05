//! Memformat angka harga menjadi teks Rupiah untuk response API.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/presenters/rupiah.md)
/// Memformat Rupiah dengan pemisah ribuan titik.
pub fn formatkan_rupiah(nilai: i64) -> String {
    let tanda = if nilai < 0 { "-" } else { "" };
    format!("Rp {tanda}{}", kelompokkan_ribuan(nilai.unsigned_abs()))
}

/// Menyisipkan titik setiap tiga digit, dihitung dari kanan.
/// Menyisipkan titik antar kelompok tiga digit dari kanan.
fn kelompokkan_ribuan(nilai: u64) -> String {
    let mut angka = nilai.to_string();
    let mut posisi = angka.len();

    while posisi > 3 {
        posisi -= 3;
        angka.insert(posisi, '.');
    }

    angka
}

#[cfg(test)]
mod tests {
    use super::formatkan_rupiah;

    #[test]
    fn rupiah_memakai_pemisah_ribuan_titik() {
        assert_eq!(formatkan_rupiah(1_600_000), "Rp 1.600.000");
        assert_eq!(formatkan_rupiah(0), "Rp 0");
        assert_eq!(formatkan_rupiah(999), "Rp 999");
        assert_eq!(formatkan_rupiah(1_000), "Rp 1.000");
        assert_eq!(formatkan_rupiah(-1_600_000), "Rp -1.600.000");
    }
}
