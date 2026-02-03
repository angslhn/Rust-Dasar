// Tuples

fn main() {
    // 1. Cara Membuat Tuple
    // Format: (Tipe1, Tipe2, Tipe3)
    // Panjangnya FIXED. Setelah dibuat, tidak bisa tambah/kurang isinya.
    let user_data: (&str, i32, bool) = ("Zeroize", 25, true);

    // 2. Mengakses Isi Tuple (Pakai Titik)
    // Beda sama array yang pakai [], tuple pakai titik (.) diikuti index.
    println!("Username: {}", user_data.0); // "Zeroize"
    println!("Umur: {}", user_data.1);     // 25
    println!("Aktif: {}", user_data.2);    // true


    // 3. Destructuring (Bongkar Paket)
    // Ini cara paling enak buat ambil isinya. Mirip JavaScript.
    let (nama, umur, status) = user_data;
    println!("Halo {}, status aktif: {}", nama, status);


    // 4. Tuple Kosong (Unit Type)
    // Ditulis sebagai (). Ini mewakili "tidak ada nilai".
    // Kalau fungsi tidak mengembalikan apa-apa, sebenarnya dia mengembalikan ().
    let kosong: () = ();
}

// 5. Fitur Paling Berguna: Return Multiple Values
// Di bahasa lain susah, di Rust gampang banget balikin banyak nilai dari fungsi.
fn get_server_status() -> (i32, String) {
    (200, String::from("OK"))
}

// Cara pakainya:
// let (kode, pesan) = get_server_status();