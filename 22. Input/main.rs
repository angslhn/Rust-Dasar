// Input

use std::io;

fn main() {
    println!("Siapa nama kamu?");
    
    let mut buffer = String::new();
    
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error");

    // Shadowing variable (Menimpa nama variable biar rapi)
    // Mengubah String -> &str yang sudah bersih
    let nama = buffer.trim(); 

    println!("Nama kamu panjangnya {} karakter", nama.len());
}