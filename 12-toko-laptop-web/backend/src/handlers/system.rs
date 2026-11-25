//! Endpoint untuk pemeriksaan status server.
//! Catatan: [Buka penjelasan](../../catatan/rust/handlers/system.md)
use axum::Json;
use serde_json::{Value, json};

/// Memastikan server API dapat menerima request.
pub async fn cek_kesehatan() -> Json<Value> {
    Json(json!({"status": "ok", "service": "toko-laptop-app"}))
}
