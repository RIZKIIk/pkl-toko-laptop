# Sintaks Dasar yang Sering Muncul

> **Navigasi:** [Pusat catatan](README.md) · [HTML](html/struktur-halaman.md) · [CSS](css/README.md) · [JavaScript](js/README.md)

Halaman ini menjelaskan bentuk sintaks yang sering dipakai di folder frontend. Gunakan sebagai kamus saat membuka file source.

## HTML

| Bentuk | Arti |
| --- | --- |
| `<main id="app">...</main>` | Elemen dengan tag `main`; `id` menjadi nama unik yang dapat dicari JavaScript atau ditarget CSS. |
| `<button type="button">` | Tombol aksi. `type="button"` mencegah tombol di dalam form mengirim form tanpa sengaja. |
| `<label for="username">` | Label terhubung dengan input yang `id`-nya sama, sehingga lebih mudah digunakan dengan mouse dan pembaca layar. |
| `aria-label`, `aria-live`, `aria-expanded` | Atribut aksesibilitas untuk memberi nama kontrol atau memberi tahu teknologi bantu tentang perubahan. |
| `data-action="logout"` | Atribut data buatan aplikasi. JavaScript membaca nilainya untuk menentukan aksi. |
| `class="nav-link active"` | Elemen memiliki dua class; CSS dapat menarget `.nav-link.active`. |

## CSS

```css
.nav-link:hover { color: white; }
@media (max-width: 620px) { .sidebar { width: 90vw; } }
```

- Selector `.nav-link` memilih semua elemen dengan class tersebut; `#app` memilih elemen berdasarkan ID.
- Deklarasi di dalam `{ ... }` berbentuk `properti: nilai;`.
- `:hover`, `:focus-visible`, dan `::before` adalah pseudo-class/element untuk keadaan atau bagian khusus.
- `@media` membatasi aturan pada kondisi tertentu, misalnya lebar layar ponsel.
- `--ink: #18252f` membuat CSS custom property; `var(--ink)` membaca nilainya.
- `display: flex` mengatur satu arah tata letak; `display: grid` membagi ruang dalam baris dan kolom.
- `rem`, `%`, `vw`, `min()`, `max()`, dan `clamp()` membantu ukuran menyesuaikan konteks layar.

## JavaScript dasar

```js
const user = { username: "Rizki", role: "buyer" };
const nama = user.username;
const label = `Halo, ${nama}`;
```

- `const` membuat nama variabel yang tidak dapat di-assign ulang; nilai object/array di dalamnya masih bisa berubah.
- `{ username: ..., role: ... }` adalah object dengan pasangan key dan value.
- `${...}` menyisipkan nilai ke template literal yang ditulis dengan backtick.
- `function nama() {}` membuat fungsi; `const tambah = (a, b) => a + b` memakai arrow function.
- `export` membuat fungsi/nilai dapat dipakai file lain; `import { api } from "./shared.js"` mengambilnya.
- `return` mengirim hasil fungsi atau menghentikan fungsi saat itu.
- `===` membandingkan nilai dan tipe; `&&` berarti kedua kondisi harus benar; `||` memilih alternatif pertama yang bernilai benar.
- `?.` mengakses property atau memanggil fungsi hanya jika nilai sebelumnya ada. Contoh: `button?.focus()`.
- `??` memberi nilai cadangan hanya jika nilai di kiri `null` atau `undefined`; tidak sama dengan `||` yang juga menganggap `0` dan string kosong sebagai salah.
- `array.map(...)` mengubah setiap item, `filter(...)` memilih sebagian item, `find(...)` mengambil item pertama yang cocok, dan `reduce(...)` mengumpulkan item menjadi satu nilai.

## DOM dan event

```js
document.querySelector("#login-form");
button.addEventListener("click", (event) => { ... });
```

- `document.querySelector()` mencari elemen pertama yang cocok dengan selector CSS.
- `addEventListener()` memasang fungsi untuk event seperti `click`, `submit`, `input`, atau `keydown`.
- `event.preventDefault()` mencegah perilaku browser bawaan, misalnya reload saat form dikirim.
- `event.target` adalah elemen asal event; `closest()` mencari elemen itu sendiri atau induk terdekat yang cocok.
- `dataset.action` membaca `data-action`; `classList.add/remove/toggle()` mengubah class tanpa menulis ulang seluruh atribut.
- `innerHTML` menafsirkan string sebagai markup, jadi nilai dari API perlu diloloskan dengan `esc()` sebelum masuk ke HTML. Untuk pesan teks biasa, gunakan `textContent`.

## Request dan error

`async function` dapat memakai `await` untuk menunggu Promise. `try { ... } catch (error) { ... }` menangani kegagalan. `fetch()` mengirim request HTTP; `response.json()` membaca body JSON. Di proyek ini, helper `api()` membungkus pola tersebut dan mengubah response gagal menjadi `ApiError`.

```js
try {
  const result = await api("/api/laptops");
} catch (error) {
  reportError(error);
}
```

`JSON.stringify(object)` mengubah object JavaScript menjadi teks JSON untuk dikirim ke backend. `new URLSearchParams(location.search)` membaca query string seperti `?registered=1`.

