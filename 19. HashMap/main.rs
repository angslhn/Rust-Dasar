// HashMap

// 1. WAJIB IMPORT!
// HashMap tidak otomatis ada di 'prelude' (tidak seperti Vec atau String).
// Kita harus panggil manual dari library standar.
use std::collections::HashMap;

fn main() {
    // 2. Membuat HashMap Baru
    let mut users: HashMap<String, String> = HashMap::new();


    // 3. Menambah Data (.insert)
    // Format: .insert(Key, Value)
    // Ingat: Key harus unik! Kalau insert key yang sama, data lama KETIMPA.
    users.insert(String::from("user1"), String::from("Aang"));
    users.insert(String::from("user2"), String::from("Zeroize"));


    // 4. Mengakses Data (.get)
    // .get() mengembalikan Option (Some/None) karena bisa saja kuncinya salah.
    let key = String::from("user1");
    
    match users.get(&key) {
        Some(nama) => println!("User ditemukan: {}", nama),
        None => println!("User tidak ada di database."),
    }


    // 5. Cek apakah Key ada? (.contains_key)
    if users.contains_key("user3") {
        println!("User 3 ada!");
    } else {
        println!("User 3 belum daftar.");
    }


    // 6. Menghapus Data (.remove)
    users.remove("user1");


    // 7. Looping HashMap
    // Urutan print-nya ACAK (tidak urut seperti Vector).
    for (username, nama_asli) in &users {
        println!("User: {} = {}", username, nama_asli);
    }
    
    // 8. Update Data (Entry API) - Fitur Canggih Rust
    // "Cek dulu ada key 'admin' gak? Kalau GAK ADA, masukin 'Admin Baru'."
    // "Kalau SUDAH ADA, jangan diapa-apain."
    users.entry(String::from("admin")).or_insert(String::from("Admin Baru"));
}