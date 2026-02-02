// Pengkondisian (If, Else If, Else)

fn main() {
    let level_keamanan = 75;

    // 1. Syntax Dasar
    // - Tidak perlu kurung () di sekitar kondisi.
    // - Blok kurung kurawal {} SANGAT WAJIB, meski isinya cuma 1 baris.
    
    if level_keamanan >= 90 {
        println!("Sangat Aman");
    } else if level_keamanan >= 50 {
        println!("Cukup Aman");
    } else {
        println!("Bahaya! Perlu update.");
    }


    // 2. Strict Boolean Condition
    // Seperti yang kita bahas sebelumnya, Rust tidak menerima "truthy/falsy".
    let ada_user = 1;
    
    // if ada_user { ... } // ERROR: expected bool, found integer
    
    if ada_user != 0 { // VALID: Hasilnya boolean
        println!("User ditemukan.");
    }


    // 3. 'if' dalam Variable (Statement vs Expression)
    // Ini fitur keren Rust! Kita bisa menyimpan hasil if ke dalam variabel.
    // Syarat: Semua blok (if & else) harus mengembalikan TIPE DATA YANG SAMA.
    
    let is_admin = true;
    
    // Jika admin, nilai akses 100, jika tidak 0.
    let access_level = if is_admin {
        100 
    } else {
        0
        // "Nol" // ERROR! Karena blok if mengembalikan integer, else tidak boleh String.
    };

    println!("Access Level: {}", access_level);
}