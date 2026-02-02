// Fungsi

// 1. Fungsi Utama (Entry Point)
// Setiap program Rust wajib punya satu fungsi 'main'.
fn main() {
    sapa_user("Aang Solihin"); // Memanggil fungsi
    
    let hasil = tambah_angka(10, 5);
    println!("Hasil tambah: {}", hasil);

    let is_valid = cek_password("rahasia123");
    println!("Password valid? {}", is_valid);
}

// 2. Fungsi dengan Parameter
// Aturan: WAJIB menulis tipe data parameter.
// Konvensi nama: snake_case (huruf kecil semua dipisah underscore).
fn sapa_user(nama: &str) {
    println!("Halo, {}! Selamat coding Rust.", nama);
}

// 3. Fungsi dengan Return Value (Nilai Balik)
// Gunakan tanda panah '->' untuk memberi tahu tipe data yang dikembalikan.
fn tambah_angka(a: i32, b: i32) -> i32 {
    // Perhatikan baris di bawah ini TIDAK ADA titik koma (;)
    a + b  
    // Ini disebut "Implicit Return". 
    // Kalau pakai titik koma, dia jadi 'statement' dan tidak mengembalikan nilai.
}

// 4. Return Eksplisit
// Kamu tetap bisa pakai keyword 'return' untuk keluar lebih awal.
fn cek_password(pass: &str) -> bool {
    if pass.len() < 8 {
        return false; // Keluar dini (Early return)
    }
    
    true // Implicit return di akhir (tanpa titik koma)
}