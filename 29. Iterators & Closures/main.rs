// Iterators & Closures

/* 
   1. CLOSURES: Fungsi anonim (tanpa nama) yang bisa disimpan di variabel.
      Mirip "Arrow Function" di JS `() => {}` atau "Lambda" di Python.
      
   2. ITERATORS: Cara mengolah urutan data (array/vector) secara berantai.
      Contoh method: .map(), .filter(), .collect(), .sum().
*/

fn main() {
    // --- 1. CLOSURES DASAR ---
    // Syntax: |parameter| { body }
    let tambah_satu = |x: i32| x + 1;
    
    println!("Hasil Closure: {}", tambah_satu(5)); // Output: 6

    // --- 2. ITERATORS (Mengolah Data) ---
    let angka = vec![1, 2, 3, 4, 5];

    // A. MAP (Mengubah setiap elemen)
    // Kita ubah setiap angka jadi kuadratnya
    // .iter() = meminjam data
    let kuadrat: Vec<i32> = angka.iter()
                                 .map(|x| x * x) 
                                 .collect(); // .collect() mengubah iterator kembali jadi Vector
    
    println!("Asli: {:?}", angka);
    println!("Kuadrat (Map): {:?}", kuadrat);

    // B. FILTER (Menyaring data)
    // Ambil hanya yang genap
    // into_iter() = mengambil ownership (data asli hilang/pindah)
    let genap: Vec<i32> = angka.into_iter()
                               .filter(|x| x % 2 == 0)
                               .collect();

    println!("Hanya Genap (Filter): {:?}", genap);
    // println!("{:?}", angka); // ERROR! karena angka sudah di-consume oleh into_iter()

    // --- 3. CHAINING (Gabungan Maut) ---
    // Gabungan map, filter, dan sum dalam satu baris.
    // Kasus: Ambil angka 1-10, cari yang ganjil, kalikan 10, lalu jumlahkan totalnya.
    
    let total: i32 = (1..=10)
        .filter(|x| x % 2 != 0) // Ambil ganjil: 1, 3, 5, 7, 9
        .map(|x| x * 10)        // Kali 10: 10, 30, 50, 70, 90
        .sum();                 // Jumlahkan
        
    println!("Total Operasi Chain: {}", total); // 250
}