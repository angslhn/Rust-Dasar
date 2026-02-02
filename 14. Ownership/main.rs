// Ownership

fn main() {
    // --- KASUS 1: Data di Stack (Copy) ---
    // Tipe data sederhana (Fixed size) seperti Integer, Float, Bool, Char.
    // Karena ukurannya kecil & pasti, data ini otomatis di-COPY.
    
    let x = 5;
    let y = x; // Nilai 5 disalin ke y.
    
    println!("x: {}, y: {}", x, y); // BISA! x masih ada.


    // --- KASUS 2: Data di Heap (Move) ---
    // Tipe data kompleks (Growable) seperti String.
    // Rust TIDAK meng-copy datanya karena bisa saja ukurannya bergiga-giga (lambat).
    // Rust melakukan "MOVE" (Pindah tangan).
    
    let s1 = String::from("Halo");
    let s2 = s1; 
    
    // SAAT BARIS DI ATAS DIJALANKAN:
    // 1. Kepemilikan data "Halo" pindah dari s1 ke s2.
    // 2. s1 dianggap "mati" (invalid).
    
    // println!("s1: {}", s1); // ERROR! "value borrowed here after move"
    println!("s2: {}", s2); // AMAN! s2 adalah owner sekarang.


    // --- KASUS 3: Clone (Deep Copy) ---
    // Kalau kita BENAR-BENAR ingin dua data terpisah (duplikat), kita harus eksplisit.
    
    let s3 = String::from("Dunia");
    let s4 = s3.clone(); // Menyalin data di Heap (lebih lambat/mahal, tapi aman).
    
    println!("s3: {}, s4: {}", s3, s4); // BISA! Keduanya hidup terpisah.

} // <-- Di sini s2, s3, s4, x, y dihapus. s1 tidak diapa-apakan karena sudah mati duluan.