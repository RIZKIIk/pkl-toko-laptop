//! Snapshot transaksi simulasi yang dipakai untuk membentuk struk pembelian.
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct PurchaseReceipt {
    pub id: i64,
    pub username: String,
    pub laptop_name: String,
    pub laptop_brand: String,
    pub unit_price: i64,
    pub quantity: i32,
    pub total_price: i64,
    pub payment_method: String,
    pub payment_status: String,
    pub created_at_epoch: i64,
}
