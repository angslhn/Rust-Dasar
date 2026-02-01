// Operators

/* 
    Rust menyediakan kumpulan operator untuk aritmatika, perbandingan, dan logika.
    Aturan Emas: Operasi hanya bisa dilakukan pada dua nilai dengan TIPE DATA YANG SAMA.
*/

fn main() {
    // 1. Aritmatika (+, -, *, /, %)
    // Semua operand harus memiliki tipe data yang sama (misal i32 dengan i32).
    let tambah = 5 + 10;      // 15
    let kurang = 95.5 - 4.3;  // 91.2
    let kali = 4 * 30;        // 120
    let bagi = 56.7 / 32.2;   // 1.7608...
    let sisa_bagi = 43 % 5;   // 3 (Modulo)

    // Hati-hati dengan pembagian integer:
    let bagi_bulat = 5 / 2;   // Hasilnya 2, bukan 2.5 (karena tipe integer membuang desimal)

    // 2. Perbandingan (==, !=, >, <, >=, <=)
    // Menghasilkan tipe data boolean (true/false).
    let apakah_sama = 1 == 1;      // true
    let apakah_beda = 1 != 2;      // true
    let lebih_besar = 10 > 5;      // true

    // 3. Logika (&&, ||, !)
    // Digunakan untuk menggabungkan beberapa kondisi boolean.
    let is_secure = true;
    let is_admin = false;

    let boleh_masuk = is_secure && is_admin; // false (AND: keduanya harus true)
    let cek_akses = is_secure || is_admin;   // true (OR: salah satu true sudah cukup)
    let tidak_aman = !is_secure;             // false (NOT: membalikkan nilai)

    // 4. Compound Assignment (+=, -=, *=, /=, %=)
    // Cara cepat untuk mengubah nilai variabel (harus variabel 'mut').
    let mut score = 10;
    score += 5; // Sama dengan: score = score + 5
    println!("Score: {}", score); // Output: 15
}