// Match

fn main() {
    let kode_status = 200;

    // 1. Syntax Dasar (Mirip Switch)
    // Bedanya: Tidak butuh 'case', tidak butuh 'break'.
    // Rust otomatis berhenti setelah menemukan yang cocok.
    match kode_status {
        200 => println!("Sukses!"),
        404 => println!("Tidak Ditemukan!"),
        500 => println!("Server Error!"),
        // 2. The "Catch-All" Pattern (_)
        // Ini SANGAT PENTING. Rust mewajibkan kita menangani SEMUA kemungkinan.
        // Jika angka lain masuk (misal 403) dan tidak ada handle-nya, program ERROR saat compile.
        // '_' ini sama dengan 'default' di switch.
        _ => println!("Kode tidak dikenal"),
    }

    // 3. Match sebagai Expression (Mengembalikan Nilai)
    // Karena match mengembalikan nilai, kita bisa simpan hasilnya ke variabel.
    let pesan = match kode_status {
        200 => "OK",
        404 => "Missing",
        _ => "Unknown",
    }; // Jangan lupa titik koma di sini karena ini akhir statement 'let'.
    
    println!("Status server: {}", pesan);

    // 4. Pattern Matching yang Lebih Canggih
    let angka = 13;

    match angka {
        1 => println!("Satu"),
        2 | 3 | 5 | 7 => println!("Bilangan Prima Kecil"), // Match banyak nilai (OR)
        10..=20 => println!("Antara 10 sampai 20"),        // Match Range (Jarak)
        _ => println!("Angka lainnya"),
    }
}