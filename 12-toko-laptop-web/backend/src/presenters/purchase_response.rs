//! Presenter untuk snapshot struk transaksi pembelian simulasi.
use crate::models::PurchaseReceipt;
use serde_json::{Value, json};

pub fn struk_ke_objek(receipt: &PurchaseReceipt) -> Value {
    json!({
        "id": receipt.id,
        "nomor": format!("LK-{:08}", receipt.id),
        "username": receipt.username,
        "nama_laptop": receipt.laptop_name,
        "merek_laptop": receipt.laptop_brand,
        "harga_satuan": receipt.unit_price,
        "jumlah": receipt.quantity,
        "total": receipt.total_price,
        "metode_pembayaran": receipt.payment_method,
        "status": receipt.payment_status,
        "dibuat_pada": receipt.created_at_epoch
    })
}
