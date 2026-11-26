// Pilih tema sebelum stylesheet tampil agar halaman tidak berkedip.
(() => {
  let savedTheme = null;
  try {
    savedTheme = localStorage.getItem("laptopku.theme");
  } catch (_) {
    // Tema sistem tetap dipakai jika browser membatasi localStorage.
  }

  const theme =
    savedTheme === "light" || savedTheme === "dark"
      ? savedTheme
      : window.matchMedia("(prefers-color-scheme: dark)").matches
        ? "dark"
        : "light";
  document.documentElement.dataset.theme = theme;
  document
    .querySelector('meta[name="theme-color"]')
    ?.setAttribute("content", theme === "dark" ? "#101719" : "#f3f6f6");
})();
