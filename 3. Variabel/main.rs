// Variabel

/* 
   Variabel di Rust secara default adalah immutable (tidak bisa diubah) demi keamanan memori.
   Ada 4 konsep utama penyimpanan nilai:
    1. Immutable (let): Default. Nilai terkunci selamanya.
    2. Mutable (mut): Eksplisit. Nilai boleh berubah, tapi tipe data harus sama.
    3. Constant (const): Nilai konstan yang "hardcoded" saat kompilasi. Wajib tulis tipe data.
    4. Shadowing: Menimpa variabel lama dengan nama yang sama (bahkan bisa ganti tipe data).
*/

fn main() {
    // --- 1. Immutable (Default) ---
    let x = 5;
    println!("Nilai x: {}", x);
    
    // x = 10; 
    // ^ JIKA BARIS DI ATAS DI-UNCOMMENT, AKAN ERROR! 
    // (Error: cannot assign twice to immutable variable)

    // --- 2. Mutable (Bisa Diubah) ---
    let mut y = 5; // Tambahkan 'mut'
    println!("Nilai y awal: {}", y);
    
    y = 10; // Berhasil diubah
    println!("Nilai y baru: {}", y);


    // --- 3. Constant ---
    // Wajib kapital dan wajib mendefinisikan tipe data (: u32).
    // Nilainya harus sudah jelas sebelum program jalan (compile-time).
    const MAX_POINTS: u32 = 100_000;
    println!("Nilai Constant: {}", MAX_POINTS);


    // --- 4. Shadowing ---
    // Mendeklarasikan ulang variabel dengan nama yang SAMA persis.
    let z = 5;
    let z = z + 1; // z yang baru (6) menutupi z yang lama (5)
    let z = z * 2; // z yang baru (12) menutupi z yang lama (6)

    println!("Nilai z akhir: {}", z); // Output: 12

    // Shadowing juga bisa untuk MENGUBAH TIPE DATA
    let spaces = "   ";       // Tipe: String text
    let spaces = spaces.len(); // Tipe: Integer (panjang text)
    // ^ Ini legal dengan Shadowing. Kalau pakai 'mut', ini akan error.
    
    println!("Jumlah spasi: {}", spaces);
}