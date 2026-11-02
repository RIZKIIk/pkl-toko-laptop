use std::io;

// Fungsi untuk menerima input
fn input() -> String {
    let mut data = String::new();

    io::stdin().read_line(&mut data).unwrap();

    data.trim().to_string()
}

fn main() {
    println!("=== TOKO LAPTOP SEDERHANA ===");

    // Menampilkan menu
    println!("1. AXIOO HYPE 5 - Rp6.000.000");
    println!("2. ROG - Rp50.000.000");
    println!("3. ASUS - Rp10.000.000");
    println!("4. Bayar");

    // Meminta nama pembeli
    println!("Nama:");
    let nama = input();
    println!("Nama pembeli siapa?: {}", nama);

    // Menyimpan total harga
    let mut total = 0;

    loop {
        println!("\nPilih menu:");
        let pilihan = input();

        // Mengecek pilihan
        if pilihan == "1" {
            total += 6000000;
            println!("AXIOO HYPE 5 ditambahkan.");
        } else if pilihan == "2" {
            total += 50000000;
            println!("ROG ditambahkan.");
        } else if pilihan == "3" {
            total += 10000000;
            println!("ASUS ditambahkan.");
        } else if pilihan == "4" {
            break;
        } else {
            println!("Pilihan tidak tersedia.");
        }
    }

    // Menampilkan hasil
    println!("\n=== STRUK ===");
    println!("Nama: {}", nama);
    println!("Total: Rp{}", total);
    println!("Terima kasih!");
}
