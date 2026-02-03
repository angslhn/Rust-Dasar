// Arrays

fn main() {
    // 1. Cara Membuat Array Biasa
    // Tipe data otomatis ditebak: [i32; 5] -> [Tipe; Jumlah]
    let angka = [10, 20, 30, 40, 50]; 

    // 2. Cara Membuat Array dengan Tipe Data Eksplisit
    // Format: [Type; Length]
    let hari: [&str; 3] = ["Senin", "Selasa", "Rabu"];

    // 3. The "Repeat" Syntax (Isi Otomatis)
    // Mau bikin array isi 0 sebanyak 500 biji? Pakai cara ini.
    // [NilaiAwal; JumlahPengulangan]
    let buffer = [0; 500]; // Isinya [0, 0, 0, ... 0]


    // 4. Mengakses Elemen
    // Menggunakan index, mulai dari 0.
    let pertama = angka[0];
    let kedua = angka[1];
    println!("Angka pertama: {}", pertama);


    // 5. Mengubah Elemen (Harus 'mut')
    let mut skor = [90, 85, 95];
    skor[1] = 100; // Mengubah index 1
    println!("Skor baru: {:?}", skor); 
    // Catatan: ":?" digunakan untuk debug print array secara utuh.


    // 6. Mengetahui Panjang Array
    println!("Panjang array: {}", hari.len());


    // 7. Looping Array
    // Menggunakan referensi (&) agar data array tidak "pindah" (moved).
    for h in hari.iter() {
        println!("Hari: {}", h);
    }
    
    // 8. Slicing (Memotong Array)
    // Kita bisa mengambil sebagian array. Hasilnya adalah 'Slice' (&[T]).
    let potong = &angka[1..4]; // Mengambil index 1, 2, 3 (4 tidak ikut)
    println!("Potongan: {:?}", potong); // Output: [20, 30, 40]
}