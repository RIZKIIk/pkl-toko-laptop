use std::io::{self, Write};

struct Laptop {
    id: usize,
    nama: String,
    merek: String,
    harga: i64,
    stok: i32,
}

fn input(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;

    let mut nilai = String::new();
    if io::stdin().read_line(&mut nilai)? == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Input berakhir.",
        ));
    }

    Ok(nilai.trim().to_string())
}

fn input_i64(prompt: &str) -> io::Result<i64> {
    loop {
        match input(prompt)?.parse() {
            Ok(nilai) => return Ok(nilai),
            Err(_) => println!("Masukkan angka yang valid."),
        }
    }
}

fn input_i32(prompt: &str) -> io::Result<i32> {
    loop {
        match input(prompt)?.parse() {
            Ok(nilai) => return Ok(nilai),
            Err(_) => println!("Masukkan angka yang valid."),
        }
    }
}

fn input_id(prompt: &str) -> io::Result<usize> {
    loop {
        match input(prompt)?.parse() {
            Ok(id) => return Ok(id),
            Err(_) => println!("Masukkan ID berupa angka yang valid."),
        }
    }
}

fn main() -> io::Result<()> {
    let mut laptops: Vec<Laptop> = Vec::new();
    let mut id_berikutnya = 1;

    loop {
        println!("\n=== CRUD TOKO LAPTOP (MEMORI) ===");
        println!("1. Tambah laptop");
        println!("2. Lihat semua laptop");
        println!("3. Update laptop");
        println!("4. Hapus laptop");
        println!("5. Keluar");

        match input("Pilih menu: ")?.as_str() {
            "1" => {
                let laptop = Laptop {
                    id: id_berikutnya,
                    nama: input("Nama  : ")?,
                    merek: input("Merek : ")?,
                    harga: input_i64("Harga : ")?,
                    stok: input_i32("Stok  : ")?,
                };
                id_berikutnya += 1;
                laptops.push(laptop);
                println!("Laptop berhasil ditambahkan.");
            }
            "2" => lihat(&laptops),
            "3" => {
                let id = input_id("ID laptop: ")?;
                match laptops.iter_mut().find(|laptop| laptop.id == id) {
                    Some(laptop) => {
                        laptop.nama = input("Nama baru  : ")?;
                        laptop.merek = input("Merek baru : ")?;
                        laptop.harga = input_i64("Harga baru : ")?;
                        laptop.stok = input_i32("Stok baru  : ")?;
                        println!("Data laptop berhasil diupdate.");
                    }
                    None => println!("ID laptop tidak ditemukan."),
                }
            }
            "4" => {
                let id = input_id("ID laptop: ")?;
                if let Some(posisi) = laptops.iter().position(|laptop| laptop.id == id) {
                    laptops.remove(posisi);
                    println!("Data laptop berhasil dihapus.");
                } else {
                    println!("ID laptop tidak ditemukan.");
                }
            }
            "5" => {
                println!("Program selesai.");
                return Ok(());
            }
            _ => println!("Pilihan tidak tersedia."),
        }
    }
}

fn lihat(laptops: &[Laptop]) {
    if laptops.is_empty() {
        println!("Belum ada data laptop.");
        return;
    }

    for laptop in laptops {
        println!(
            "{}. {} | {} | Rp{} | stok {}",
            laptop.id, laptop.nama, laptop.merek, laptop.harga, laptop.stok
        );
    }
}
