// Random

/* 
    Rust tidak menyertakan fungsi random di standard library (std) demi keamanan 
    dan performa agar core bahasa tetap kecil.
    Kita menggunakan crate eksternal paling populer bernama "rand".
    
    Konsep: Kita butuh "Generator" (rng) untuk menghasilkan nilai acak.
*/

use rand::Rng; // Wajib import Trait ini agar method .gen() bisa dipakai

fn main() {
    // Inisialisasi thread_rng (Random Number Generator lokal)
    let mut rng = rand::thread_rng();

    // --- 1. RANDOM NUMBER RANGE ---
    
    // Generate angka integer antara range tertentu
    // Format: gen_range(min..max) -> max TIDAK ikutan (exclusive)
    // Format: gen_range(min..=max) -> max IKUTAN (inclusive)
    
    let acak_1_100 = rng.gen_range(1..101); // 1 sampai 100
    println!("Angka Hoki (1-100): {}", acak_1_100);

    let dadu = rng.gen_range(1..=6); // 1 sampai 6
    println!("Lempar Dadu: {}", dadu);

    // --- 2. RANDOM TIPE LAIN ---

    // A. Random Float (0.0 sampai 1.0)
    // Mirip Math.random() di Javascript
    let acak_float: f64 = rng.gen();
    println!("Float acak (0.0-1.0): {}", acak_float);

    // B. Random Boolean (Coin Flip)
    // gen_bool(p) dimana p adalah probabilitas true (0.0 - 1.0)
    let is_head = rng.gen_bool(0.5); // 50% kemungkinan true
    println!("Koin (Head/Tail): {}", if is_head { "Head" } else { "Tail" });

    // --- 3. PASSWORD GENERATOR SIMPEL ---
    // Mengambil karakter acak dari array char
    let chars = ['A', 'B', 'C', '1', '2', '3', '!', '@'];
    
    // Mengambil index acak
    let random_index = rng.gen_range(0..chars.len());
    println!("Karakter acak terpilih: {}", chars[random_index]);
}