// Async & Await (Non-Blocking I/O)


/*
  Menginstall library eksternal executor asynchronous
  - cargo add tokio --features full
*/

/* KONSEP:
   1. Synchronous (Biasa): Pesan kopi -> Tunggu dibuat -> Baru bisa duduk. (Blokir waktu)
   2. Asynchronous (Async): Pesan kopi -> Cari tempat duduk -> Kopi jadi, dipanggil. (Efisien)
   
   SYARAT:
   - Harus pakai Executor (disini kita pakai "Tokio").
   - Fungsi ditandai dengan keyword `async`.
   - Panggil fungsi pakai `.await`.
*/

use tokio::time::{sleep, Duration};

// Ubah fn main() biasa menjadi async main dengan macro Tokio
#[tokio::main]
async fn main() {
    println!("--- MULAI PROGRAM ---");
    let start = std::time::Instant::now();

    // --- 1. CONTOH SALAH (Sequential / Berurutan) ---
    // Ini async, tapi dijalankan satu-satu. Tidak ada bedanya sama coding biasa.
    // print_satu().await;
    // print_dua().await; 


    // --- 2. CONTOH BENAR (Concurrent / Barengan) ---
    // Kita gunakan `tokio::join!` untuk menjalankan beberapa tugas async SEKALIGUS.
    
    let proses_1 = ambil_data_database(); // Return "Future" (belum jalan)
    let proses_2 = kirim_email();         // Return "Future" (belum jalan)

    println!("Sedang memproses keduanya...");
    
    // .await disini menunggu KEDUANYA selesai
    let (hasil_db, hasil_email) = tokio::join!(proses_1, proses_2);

    println!("Hasil DB: {}", hasil_db);
    println!("Hasil Email: {}", hasil_email);

    let duration = start.elapsed();
    println!("--- SELESAI DALAM: {:?} ---", duration);
    // Perhatikan: Waktunya adalah waktu proses terlama (misal 2 detik), 
    // BUKAN total penjumlahan (2 + 1 detik).
}

// --- FUNGSI ASYNC ---

async fn ambil_data_database() -> String {
    println!("> Mulai query database (butuh 2 detik)...");
    
    // PENTING: Jangan pakai std::thread::sleep! Itu memblokir seluruh thread.
    // Pakai tokio::time::sleep agar thread bisa mengerjakan hal lain sambil nunggu.
    sleep(Duration::from_secs(2)).await; 
    
    println!("> Query database selesai!");
    String::from("Data User: Aang")
}

async fn kirim_email() -> String {
    println!("> Mulai kirim email (butuh 1 detik)...");
    sleep(Duration::from_secs(1)).await;
    println!("> Email terkirim!");
    String::from("Email Sent: Success")
}

/* CATATAN PENTING TENTANG "FUTURE":
   Di Rust, memanggil fungsi async (misal `kirim_email()`) TIDAK langsung menjalankan kode.
   Dia hanya mengembalikan tiket janji bernama "Future".
   
   Kode baru benar-benar jalan kalau tiket itu di `.await` atau dimasukkan ke `tokio::join!`.
   Ini disebut "Lazy Evaluation".
*/