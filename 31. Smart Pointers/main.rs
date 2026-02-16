// Smart Pointers

/* 
    Pointer biasa hanya meminjam alamat memori (&).
   Smart Pointers "memiliki" datanya dan mengatur kapan dihapus.
   
   Jenis utama:
   1. Box<T> -> Simpan data di Heap (bukan Stack). Wajib buat data rekursif.
   2. Rc<T>  -> Reference Counting. Satu data dimiliki BANYAK variabel.
   3. RefCell<T> -> Interior Mutability. Mengubah data walau variabelnya immutable.
*/

use std::rc::Rc;
use std::cell::RefCell;

// Contoh Kasus Box: Definisi List Berantai (Recursive Type)
// Tanpa Box, Rust bingung ukuran memorinya karena List bisa terus bersarang.
enum List {
    Isi(i32, Box<List>), // List berisi List lagi (infinite size kalau di stack)
    Kosong,
}

fn main() {
    // --- 1. BOX<T> (Alokasi Heap) ---
    let b = Box::new(5); // Angka 5 disimpan di Heap, b adalah pointer di Stack
    println!("b = {}", b);
    
    // Implementasi Recursive List
    use List::{Isi, Kosong};
    let list = Isi(1, Box::new(Isi(2, Box::new(Kosong))));
    

    // --- 2. Rc<T> (Reference Counting - Multi Ownership) ---
    // Aturan Rust: 1 Data = 1 Pemilik. Rc mematahkan aturan ini (single thread only).
    
    let data_shared = Rc::new(String::from("Data Bersama"));
    
    // Clone disini BUKAN copy data, tapi nambah counter referensi.
    let pemilik_1 = Rc::clone(&data_shared);
    let pemilik_2 = Rc::clone(&data_shared);

    println!("Pemilik 1: {}", pemilik_1);
    println!("Pemilik 2: {}", pemilik_2);
    // Data asli baru dihapus kalau semua pemilik (count) sudah hilang.
    println!("Jumlah pemilik sekarang: {}", Rc::strong_count(&data_shared));


    // --- 3. RefCell<T> (Interior Mutability - Cheat Code) ---
    // Mengubah data didalam variabel immutable.
    // Ini mengecek aturan borrowing saat RUNTIME, bukan COMPILE TIME.
    // Hati-hati: Bisa bikin panic saat runtime kalau salah pakai.

    let hp = RefCell::new(String::from("Nokia"));

    {
        // borrow_mut() meminjam akses tulis
        let mut akses_tulis = hp.borrow_mut();
        akses_tulis.push_str(" 3310");
    } // akses_tulis scope-nya habis disini

    // Sekarang bisa dibaca
    println!("HP saya: {}", hp.borrow()); 
}