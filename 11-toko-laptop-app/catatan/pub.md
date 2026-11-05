# Visibility (`pub`) di Rust

Di Rust, `pub` menentukan siapa yang boleh memakai suatu item, seperti fungsi, struct, atau field. Bentuk `pub(super)` dan `pub(crate)` membatasi akses supaya item tidak terbuka lebih luas dari yang diperlukan.

1. **`pub`**: bisa diakses dari luar modul. Kalau item berada di dalam library, item itu bisa menjadi bagian dari API untuk pengguna library—selama modul-modul di jalurnya juga terbuka.
2. **`pub(crate)`**: bisa diakses dari mana saja di dalam crate, termasuk modul yang bersaudara. Item ini tidak dibuka untuk pengguna crate dari luar.
3. **`pub(super)`**: bisa diakses dari modul induknya dan turunannya. Contohnya, di `handlers::laptops`, induknya adalah `handlers`; modul `create` dan `read` yang menjadi anak `laptops` juga bisa memakai fungsi tersebut.
4. **Tanpa `pub`**: hanya tersedia di modul itu dan turunannya.

## Contoh dari proyek

- `success_response` di `handlers/laptops/mod.rs` memakai `pub(super)`, sehingga helper itu tersedia di bagian handler terkait tanpa menjadi API umum aplikasi.
- `verify_token` di `auth/jwt.rs` memakai `pub(crate)` karena dipakai oleh bagian lain di dalam crate, tetapi tidak perlu dibuka untuk pemakai dari luar.

## Kenapa tidak selalu pakai `pub`?

`pub` bisa membuat item dapat diakses lebih luas dari yang dibutuhkan. Memilih tingkat akses yang lebih sempit menjaga bagian internal tetap internal dan mengurangi ketergantungan antarbagian.

```rust
pub(super) fn nama_fn() {}
```
