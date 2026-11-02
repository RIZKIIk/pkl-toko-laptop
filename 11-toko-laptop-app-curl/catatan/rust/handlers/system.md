# `handlers/system.rs`

> **Navigasi:** [Indeks catatan](../../README.md) · [README proyek](../../../README.md) · [Endpoint API](../../../endpoin.md)

[Buka source `system.rs`](../../../src/handlers/system.rs)

## Tujuan

Berisi endpoint untuk memastikan server HTTP merespons. Fungsi ini tidak mengubah data.

## Import

- `Json` membungkus body response JSON.
- `Value` dan `json!` membentuk object JSON.

## Fungsi

### `health()`

Tidak menerima state atau request body. Fungsi ini langsung mengembalikan `{"status":"ok","service":"toko-laptop-app"}`. Karena tidak mengakses database, endpoint tetap berguna untuk memastikan proses HTTP merespons.

## Route

Fungsi ini dipasang sebagai `GET /health` di [routes.rs](../routes.md).
