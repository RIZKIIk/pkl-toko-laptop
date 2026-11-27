# `login.js` — Masuk ke Akun

> **Navigasi:** [Indeks JavaScript](README.md) · [Source](../../js/login.js) · [Helper API](shared.md) · [Form daftar](register.md)

`renderLogin()` membuat halaman masuk, lalu menambahkan event untuk tombol lihat password dan submit form.

## Membentuk halaman

- `document.title` mengubah judul tab.
- `new URLSearchParams(location.search)` membaca status query string seperti `?session=expired` atau `?registered=1`.
- Template literal ber-tag `html` menyisipkan komponen brand dan tombol tema. Class `visible` dan `success` dipilih sesuai query.
- `label for` menghubungkan label dengan input. `autocomplete` membantu browser mengisi kredensial dengan benar.
- `role="alert"` menandai pesan penting yang perlu diumumkan teknologi bantu.
- Ikon user dan gembok pada input dibuat dari SVG inline dekoratif (`aria-hidden="true"`); label form tetap menjadi nama input yang dibaca teknologi bantu.

## Validasi dan request

Handler `submit` memanggil `event.preventDefault()` agar browser tidak reload. Form membaca `form.elements.username.value` dan password, lalu `trim()` menghapus spasi tepi username. Jika ada kolom kosong, pesan ditampilkan dan fokus dikirim ke input yang perlu diisi.

Saat valid, tombol dikunci dan labelnya diganti menjadi status loading. `api("/api/auth/login", ...)` mengirim `POST` dengan JSON berisi username/password. `auth: false` menyatakan request login tidak memakai token lama.

Response diperiksa agar `access_token`, `username`, dan `role` tersedia. Token disimpan lewat `saveToken()`, lalu `location.assign("/dashboard")` membuka dashboard. `catch` menampilkan pesan error dan mengaktifkan tombol lagi.

## Tombol password

Event click mengganti `input.type` dari `password` ke `text` atau sebaliknya. Teks tombol dan `aria-label` ikut diperbarui agar tindakan berikutnya terbaca jelas.

Panel promosi menggunakan gambar SVG lokal `/assets/images/laptopku-hero.svg` dengan `alt=""` karena ilustrasi hanya dekoratif. Pada layar sempit panel tersebut disembunyikan, sedangkan logo brand dipindahkan ke baris atas form.

