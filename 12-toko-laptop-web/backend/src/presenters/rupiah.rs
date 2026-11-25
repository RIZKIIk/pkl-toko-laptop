//! Memformat angka harga menjadi teks Rupiah untuk response API.
//! Catatan: [Buka penjelasan](../../catatan/rust/presenters/rupiah.md)
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
