// Date and Time

/* 
    Standard library Rust (std::time) hanya bagus untuk durasi/stopwatch.
    Untuk Kalender, Jam Dinding, dan Zona Waktu, standar industrinya adalah crate "chrono".
*/

use chrono::{Local, Utc, Datelike, Timelike}; 
// Local = Waktu Komputer, Utc = Waktu Global (Greenwich)

fn main() {
    // --- 1. MENDAPATKAN WAKTU SEKARANG ---
    
    let now = Local::now(); // Mengambil waktu sistem saat ini
    println!("Waktu Mentah: {}", now); // Format ISO 8601

    // --- 2. FORMATTING (Mempercantik Tampilan) ---
    // Menggunakan kode format (strftime)
    // %Y = Tahun, %m = Bulan, %d = Tanggal
    // %H = Jam (24), %M = Menit, %S = Detik, %A = Nama Hari
    
    let format_indo = now.format("%A, %d-%m-%Y %H:%M WIB");
    println!("Waktu Cantik: {}", format_indo);

    // --- 3. MENGAMBIL BAGIAN SPESIFIK ---
    // Membutuhkan trait Datelike dan Timelike diatas
    
    println!("--- Detail ---");
    println!("Tahun saja : {}", now.year());
    println!("Bulan saja : {}", now.month());
    println!("Jam saja   : {}", now.hour());

    // --- 4. UTC vs LOCAL ---
    let waktu_utc = Utc::now();
    println!("Waktu Jakarta (Local): {}", now.format("%H:%M"));
    println!("Waktu London  (UTC)  : {}", waktu_utc.format("%H:%M"));
    
    // --- 5. PERHITUNGAN WAKTU ---
    // Misal: Tanggal 2 minggu lagi dari sekarang
    // checked_add_days membutuhkan fitur chrono yang agak advanced, 
    // tapi kita bisa pakai Duration sederhana untuk jam/menit
    
    use chrono::Duration;
    
    let dua_jam_lagi = now + Duration::hours(2);
    println!("Dua jam lagi adalah: {}", dua_jam_lagi.format("%H:%M"));
}