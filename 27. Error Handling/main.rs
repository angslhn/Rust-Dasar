// Error Handling (Penanganan Error)

/* 
  Rust tidak punya "Try-Catch" (seperti JS/Java) dan tidak punya "Null".  
    Rust membagi error menjadi 2:
    1. Unrecoverable (Fatal): Program langsung berhenti (Crash). Contoh: panic!
    2. Recoverable (Bisa ditangani): Program lanjut jalan. Contoh: Result<T, E>
*/

fn main() {
    // --- 1. UNRECOVERABLE ERROR (PANIC) ---
    // Gunakan ini jika program benar-benar rusak dan tidak aman untuk lanjut.
    // panic!("Program meledak!"); // Uncomment baris ini untuk melihat efeknya
    

    // --- 2. RECOVERABLE ERROR (RESULT) ---
    /*
       Enum Result punya dua kemungkinan:
       - Ok(value)  -> Berhasil, isinya data.
       - Err(error) -> Gagal, isinya pesan error.
    */

    let hasil_bagi = bagi_angka(10.0, 2.0);

    // CARA A: Menggunakan `match` (Cara paling aman & manual)
    // Kita harus menangani kedua kemungkinan secara eksplisit.
    match hasil_bagi {
        Ok(nilai) => println!("Berhasil! Hasilnya: {}", nilai),
        Err(pesan) => println!("Gagal! Errornya: {}", pesan),
    }

    // Coba case error (bagi 0)
    let error_bagi = bagi_angka(5.0, 0.0);
    match error_bagi {
        Ok(_) => println!("Aman"),
        Err(e) => println!("Waduh error: {}", e), // Akan print ini
    }


    // --- 3. SHORTCUTS (UNWRAP & EXPECT) ---
    // Gunakan hanya jika kamu YAKIN 100% tidak akan error, 
    // atau di tahap prototyping cepat.
    
    // .unwrap() -> Jika Ok, ambil nilainya. Jika Err, langsung PANIC (Crash).
    let angka = bagi_angka(100.0, 10.0).unwrap(); 
    println!("Unwrap: {}", angka);

    // .expect() -> Sama kayak unwrap, tapi kita bisa kasih pesan custom saat crash.
    // let crash = bagi_angka(5.0, 0.0).expect("Woy jangan bagi nol!"); 
    

    // --- 4. OPERATOR TANDA TANYA (?) ---
    // Ini teknik advanced untuk "melempar" error ke fungsi diatasnya.
    // Hanya bisa dipakai di dalam fungsi yang mengembalikan Result.
    // Lihat fungsi `fungsi_complex` di bawah.
}

// --- FUNGSI PEMBANTU ---

// Fungsi ini mengembalikan Result. 
// Jika sukses return f64, jika gagal return String.
fn bagi_angka(pembilang: f64, penyebut: f64) -> Result<f64, String> {
    if penyebut == 0.0 {
        // Return Error
        return Err(String::from("Tidak bisa membagi dengan nol"));
    }
    // Return Success
    Ok(pembilang / penyebut)
}

/* Contoh penggunaan operator `?`
   Jika `bagi_angka` error, fungsi ini akan langsung berhenti dan me-return error tersebut.
   Jika `bagi_angka` sukses, nilainya diambil dan lanjut ke baris berikutnya.
*/
fn _fungsi_complex() -> Result<f64, String> {
    let a = bagi_angka(10.0, 2.0)?; // Jika error, return Err disini. Jika ok, a = 5.0
    let b = bagi_angka(a, 0.0)?;    // Error disini, fungsi berhenti & return Err
    
    Ok(b) // Baris ini tidak akan pernah dieksekusi
}