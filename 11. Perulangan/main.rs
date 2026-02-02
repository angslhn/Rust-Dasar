// Perulangan

fn main() {
    // 1. loop (The Infinite Loop)
    // Ini perulangan paling dasar. Dia akan jalan SELAMANYA sampai ketemu 'break'.
    // Cocok untuk server yang harus standby terus (seperti server Zeroize nanti).
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Looping ke-{}", counter);

        if counter == 3 {
            println!("Berhenti di 3!");
            break; // Keluar dari loop
        }
    }

    // Fitur Spesial: Mengembalikan Nilai dari 'loop'
    // Kita bisa menjadikan loop sebagai expression untuk mengisi variabel.
    let hasil = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // Return nilai 20
        }
    };
    println!("Hasil return loop: {}", hasil);


    // 2. while (Conditional Loop)
    // Jalan SELAMA kondisinya 'true'. Mirip banget sama JavaScript.
    let mut nyawa = 3;
    while nyawa > 0 {
        println!("Sisa nyawa: {}", nyawa);
        nyawa -= 1;
    }
    println!("Game Over!");


    // 3. for (The Iterator Loop) -> Paling Sering Dipakai
    // Paling aman & cepat. Kita tidak perlu mikirin index manual (i++).
    
    // a. Menggunakan Range (Jarak)
    // 1..5  artinya 1 sampai 4 (5 tidak ikut)
    // 1..=5 artinya 1 sampai 5 (5 ikut)
    for angka in 1..=5 {
        println!("Hitung: {}", angka);
    }

    // b. Iterasi Array (Membaca isi list)
    let password_list = ["admin123", "rahasia", "qwerty"];
    
    // .iter() digunakan agar kita cuma 'meminjam' datanya, bukan mengambil hak milik.
    for pass in password_list.iter() {
        println!("Cek password: {}", pass);
    }
}