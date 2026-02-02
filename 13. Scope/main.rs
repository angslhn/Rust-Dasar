// Scope (Jangkauan)

// 1. Global Scope (Didefinisikan dengan 'const' atau 'static')
// Bisa diakses di mana saja.
const SERVER_PORT: u32 = 8080; 

fn main() {
    // 2. Local Scope (Block Scope)
    // Variabel hanya hidup di dalam kurung kurawal {} tempat dia dibuat.
    
    let outer = "Saya di luar";
    
    { // Mulai Scope Baru (Inner Scope)
        let inner = "Saya di dalam";
        
        println!("Akses outer dari dalam: {}", outer); // BISA
        println!("Akses inner dari dalam: {}", inner); // BISA
    } // <-- STOP! Di sini 'inner' DIHANCURKAN (Dropped) dari memori.

    println!("Akses outer dari luar: {}", outer); // BISA
    // println!("Akses inner dari luar: {}", inner); // ERROR! 'inner' not found.


    // 3. Variable Shadowing (Menimpa Nama)
    // Di Rust, kita boleh membuat variabel dengan nama yang SAMA di scope yang berbeda (atau sama).
    // Ini disebut 'Shadowing'.
    
    let gaji = 5_000_000;
    
    {
        // Kita bikin variabel baru dengan nama sama.
        // Ini TIDAK mengubah 'gaji' yang di luar, tapi membuat variabel baru yang menutupi.
        let gaji = 10_000_000; 
        println!("Gaji di scope dalam: {}", gaji); // 10 juta
    }
    
    // Kembali ke scope luar, variabel asli masih aman
    println!("Gaji di scope luar: {}", gaji); // Tetap 5 juta
}