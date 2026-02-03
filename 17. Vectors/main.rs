// Vectors

fn main() {
    // 1. Cara Membuat Vector
    // Cara A: Pakai function new()
    let mut v: Vec<i32> = Vec::new(); // Harus explisit tipe datanya kalau kosong
    
    // Cara B: Pakai Macro vec![] (Paling sering dipakai)
    // Rust otomatis tahu ini isinya i32.
    let mut angka = vec![1, 2, 3]; 


    // 2. Menambah Data (Push)
    // Syarat: Variabel harus 'mut' (mutable).
    v.push(10);
    v.push(20);
    v.push(30);
    
    angka.push(4); // [1, 2, 3, 4]


    // 3. Menghapus Data Terakhir (Pop)
    // pop() mengembalikan nilai Option (Some(nilai) atau None jika kosong).
    let terakhir = angka.pop(); // Menghapus 4
    println!("Yang dihapus: {:?}", terakhir); // Output: Some(4)


    // 4. Mengakses Data dengan AMAN (.get)
    // Kalau pakai index [] seperti array:
    // let bahaya = v[100]; // PANIC (Crash) kalau index 100 gak ada.

    // Cara Aman (Rust Style):
    match v.get(100) {
        Some(nilai) => println!("Nilainya: {}", nilai),
        None => println!("Data tidak ditemukan!"),
    }


    // 5. Iterasi (Looping)
    // Kita pakai &v agar vector tidak "pindah tangan" (moved) dan hilang.
    for i in &v {
        println!("Angka: {}", i);
    }
    
    // 6. Mengubah nilai saat iterasi
    let mut saldo = vec![100, 200, 300];
    for s in &mut saldo {
        *s += 50; // Menambah 50 ke setiap saldo (* adalah dereference)
    }
    println!("Saldo baru: {:?}", saldo);
}