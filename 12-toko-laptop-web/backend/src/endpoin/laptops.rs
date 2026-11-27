//! Endpoint katalog dan operasi inventaris laptop.
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::handlers::laptops;

pub(super) fn router() -> Router<PgPool> {
    Router::<PgPool>::new()
        .route(
            "/api/laptops",
            get(laptops::daftar_laptop).post(laptops::tambah_laptop),
        )
        .route(
            "/api/laptops/{id}",
            get(laptops::ambil_laptop)
                .patch(laptops::ubah_laptop)
                .delete(laptops::hapus_laptop),
        )
        .route("/api/laptops/{id}/buy", post(laptops::beli_laptop))
}
