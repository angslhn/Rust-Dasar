// String

fn main() {
    // 1. String Slice (&str) - "The Static Guest"
    // - Biasanya bersifat 'immutable' (tidak bisa diubah).
    // - Disimpan di memori program (Stack/Binary).
    // - Sangat cepat karena ukurannya sudah pasti saat dicompile.
    let string_tetap: &str = "Aang Solihin"; 

    println!("Nama lengkap saya adalah {}", string_tetap);

    // 2. String Object (String) - "The Growable Owner"
    // - Bisa diubah (mutable), bisa ditambah, bisa dikurang.
    // - Disimpan di 'Heap' (memori dinamis).
    // - Berguna untuk data yang kita tidak tahu panjangnya (misal: input user).
    let mut string_dinamis: String = String::from("Halo");
    
    // Menambahkan karakter atau kata
    string_dinamis.push('!');         // push untuk 1 karakter (char)
    string_dinamis.push_str(" Dunia"); // push_str untuk string slice (&str)
    
    println!("{}", string_dinamis); // Output: Halo! Dunia

    // 3. Konversi (Pindah jalur)
    // Dari &str ke String
    let s = "teks".to_string();
    let s2 = String::from("teks");

    // Dari String ke &str (Dikenal dengan istilah 'Deref Coercion')
    let slice: &str = &string_dinamis; 

    // 4. Manipulasi Dasar
    let teks = String::from("Rust itu asik");
    
    println!("Panjang: {} bytes", teks.len());
    println!("Apakah kosong? {}", teks.is_empty());
    println!("Ganti kata: {}", teks.replace("asik", "keren"));
}