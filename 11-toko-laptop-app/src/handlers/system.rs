//! Endpoint untuk pemeriksaan status server.
//! Catatan: [Buka penjelasan](file:///home/rizki/projects/Rust/tugas/PKL/11-toko-laptop-app/catatan/rust/handlers/system.md)
use axum::Json;
use serde_json::{Value, json};

/// Memastikan server API dapat menerima request.
pub async fn health() -> Json<Value> {
    Json(json!({"status": "ok", "service": "toko-laptop-app"}))
}
