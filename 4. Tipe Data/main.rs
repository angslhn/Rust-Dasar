// Data Types

/* 
    Rust adalah bahasa "statically typed", artinya compiler harus mengetahui tipe data 
    dari semua variabel saat kompilasi agar memori aman.
   
    Ada dua kategori utama tipe data di Rust:
      1. Scalar Types: Mewakili satu nilai tunggal (Integer, Float, Boolean, Char).
      2. Compound Types: Menggabungkan beberapa nilai menjadi satu (Tuple, Array).
*/

fn main() {
    // --- 1. SCALAR TYPES (Nilai Tunggal) ---

    // A. Integer (Bilangan Bulat)
    // i32 = signed (bisa negatif), u32 = unsigned (hanya positif)
    // Default di Rust adalah i32.
    let x: i32 = -500; 
    let y: u32 = 500;  
    println!("Integer: {} dan {}", x, y);

    // B. Floating-Point (Desimal)
    // f32 (single precision) dan f64 (double precision).
    // Default adalah f64 (lebih presisi dan kecepatan hampir sama di CPU modern).
    let pi: f64 = 3.14159;
    println!("Float: {}", pi);

    // C. Boolean
    // Hanya bernilai 'true' atau 'false'. Biasa untuk logika if/else.
    let is_active: bool = true;
    println!("Status aktif: {}", is_active);

    // D. Character (Char)
    // WAJIB pakai tanda kutip satu (' '). Ukuran 4 bytes (Unicode).
    // Bisa simpan huruf, angka, bahkan emoji.
    let letter: char = 'A';
    let emoji: char = '🔥'; 
    println!("Char: {} {}", letter, emoji);

    // E. String vs &str (Slice)
    // 1. String Slice (&str)
    // Tipe ini paling dasar. Biasanya menunjuk ke teks yang hardcoded di program.
    // Ukurannya tetap (fixed) dan tersimpan di memori program, bukan di heap.
    // Ciri-ciri: Pakai tanda kutip dua ("...").
    let fullname: &str = "Aang Solihin";
    println!("Nama (slice): {}", fullname); 

    // 2. String Object (String)
    // Ini tipe data String yang lebih kompleks (Growable/Bisa membesar).
    // Disimpan di Heap memori. Bisa diubah isinya.
    // Biasanya dibuat dari &str menggunakan .to_string() atau String::from().
    let mut pekerjaan = String::from("Programmer");
    
    pekerjaan.push_str(" Rust"); // Menambahkan text (hanya bisa di tipe String, bukan &str)
    println!("Pekerjaan: {}", pekerjaan);

    // KESIMPULAN:
    // Gunakan &str jika teksnya fix/cuma buat baca.
    // Gunakan String jika teksnya perlu diubah/ditambah/input dari user.

    // --- 2. COMPOUND TYPES (Kumpulan Nilai) ---

    // A. Tuple
    // Kumpulan nilai dengan tipe data BEDA. Panjangnya TETAP (fixed).
    // Penulisan menggunakan kurung biasa ().
    let my_tuple: (i32, f64, char) = (500, 6.4, 'Z');
    
    // Akses data tuple menggunakan tanda titik (.) diikuti index.
    let angka = my_tuple.0;
    let huruf = my_tuple.2;
    println!("Tuple: angka {}, huruf {}", angka, huruf);

    // Destructuring Tuple (Membongkar isi tuple ke variabel terpisah)
    let (a, b, c) = my_tuple;
    println!("Destructuring b: {}", b); // Output: 6.4

    // B. Array
    // Kumpulan nilai dengan tipe data SAMA. Panjangnya TETAP (fixed).
    // Penulisan menggunakan kurung siku [].
    // Format Tipe: [TipeData; JumlahElemen]
    let hari: [&str; 3] = ["Senin", "Selasa", "Rabu"];
    
    // Akses array pakai index [], mulai dari 0.
    println!("Hari pertama: {}", hari[0]);

    // Hati-hati: Array di Rust tersimpan di Stack. 
    // Jika akses index di luar batas (misal hari[5]), program akan PANIC (Crash/Error).
}