// Concurrency (Multitasking / Threads)

/* 
    Concurrency artinya melakukan banyak hal sekaligus.
   Rust menggunakan OS Threads (1:1 model).
   
   Hati-hati: Thread berjalan INDEPENDEN. Urutan output tidak dijamin.
*/

use std::thread;
use std::time::Duration;
use std::sync::mpsc; // mpsc = Multiple Producer, Single Consumer (Saluran komunikasi)

fn main() {
    // --- 1. SPAWN THREAD (Membuat Thread Baru) ---
    // Thread utama (main) jalan sendiri, thread baru jalan sendiri.
    
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread Baru: kerja nomor {}", i);
            thread::sleep(Duration::from_millis(500)); // Tidur 0.5 detik
        }
    });

    // Main thread juga kerja
    for i in 1..3 {
        println!("Main Thread: kerja nomor {}", i);
        thread::sleep(Duration::from_millis(500));
    }

    // .join() -> Menunggu thread baru selesai sebelum program tutup.
    // Kalau tidak di-join, main thread selesai duluan, thread baru mati paksa.
    handle.join().unwrap();


    // --- 2. MOVE CLOSURE (Memindah Data ke Thread) ---
    let data = vec![1, 2, 3];

    // Kita wajib pakai keyword `move` agar kepemilikan `data` pindah ke thread baru.
    // Tanpa `move`, Rust khawatir main thread menghapus `data` saat thread baru masih pakai.
    let handle_data = thread::spawn(move || {
        println!("Data di dalam thread: {:?}", data);
    });

    handle_data.join().unwrap();


    // --- 3. CHANNELS (Komunikasi antar Thread) ---
    // Cara aman kirim pesan antar thread: Jangan berbagi memori, tapi kirim pesan!
    // tx = transmitter (pengirim), rx = receiver (penerima)
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let pesan = String::from("Halo dari seberang!");
        tx.send(pesan).unwrap(); // Kirim pesan
        // println!("{}", pesan); // ERROR! pesan sudah dikirim (ownership pindah)
    });

    // Menerima pesan di main thread
    let diterima = rx.recv().unwrap();
    println!("Pesan diterima: {}", diterima);
}