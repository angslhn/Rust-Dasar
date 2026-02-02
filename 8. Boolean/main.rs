// Boolean

fn main() {
    // 1. Deklarasi (Explisit & Implisit)
    let is_rust_fun: bool = true;      // Dengan anotasi tipe
    let is_javascript_hard = false;    // Rust menebak tipenya (inference)

    // 2. Ekspresi Logika (Hasilnya selalu Boolean)
    let a = 10;
    let b = 5;

    let is_greater = a > b;  // true
    let is_equal = a == b;   // false

    // 3. Penggunaan dalam Pengkondisian (If/Else)
    // DI RUST: Kondisi HARUS berupa boolean asli. 
    // Tidak ada konsep "truthy" atau "falsy" seperti di JS (di mana 1 dianggap true).
    
    if is_rust_fun {
        println!("Ayo lanjut belajar!");
    } else {
        println!("Mungkin perlu kopi dulu.");
    }

    // 4. Operator Logika yang Menghasilkan Boolean
    // && (AND) : True jika semua true
    // || (OR)  : True jika salah satu true
    // !  (NOT) : Membalikkan nilai
    
    let is_logged_in = true;
    let has_permission = false;

    let can_access = is_logged_in && has_permission; // false
    
    println!("Akses diberikan: {}", can_access);
}