// Structs

// 1. Mendefinisikan Struct (Blueprint)
// Gunakan CamelCase untuk nama Struct.
struct AkunZeroize {
    username: String,
    email: String,
    login_count: u64,
    active: bool,
}


// 2. Tuple Struct (Struct tanpa nama field)
// Cocok untuk membungkus data sederhana agar punya tipe sendiri.
// Misalnya: Membedakan RGBColor dengan Point3D meski isinya sama-sama 3 integer.
struct Warna(i32, i32, i32);


// 3. Struct Implementation (Menghidupkan Struct)
// Di Rust, "Data" (struct) dan "Perilaku" (impl) dipisah.
// struct = Apa isinya?
// impl   = Apa yang bisa dia lakukan?

struct Persegi {
    sisi: u32,
}

impl Persegi {
    // A. Associated Function (Static Method)
    // Ciri-ciri: TIDAK punya parameter '&self'.
    // Fungsi ini milik "Blueprint", bukan milik "Object individu".
    // Biasanya dipakai untuk Constructor (mirip 'constructor' di Class JS).
    // Cara panggil pakai '::' -> Persegi::new(10)
    fn new(sisi: u32) -> Persegi {
        Persegi { sisi }
    }

    // B. Method (Instance Method)
    // Ciri-ciri: Parameter pertamanya WAJIB '&self'.
    // '&self' artinya "instance ini" (mirip keyword 'this' di JS).
    // Fungsi ini bisa mengakses data di dalam struct (self.sisi).
    // Cara panggil pakai '.' -> p.hitung_luas()
    fn hitung_luas(&self) -> u32 {
        self.sisi * self.sisi
    }
}

fn main() {
    // 3. Membuat Instance (Membangun Object)
    // Urutan field tidak harus sama dengan definisi, asalkan namanya cocok.
    let mut user1 = AkunZeroize {
        email: String::from("aang@zeroize.com"),
        username: String::from("aang_solihin"),
        active: true,
        login_count: 1,
    };

    // 4. Mengakses & Mengubah Data (Dot Notation)
    // Syarat ubah: Variabel 'user1' harus 'mut'.
    // Kalau 'user1' tidak 'mut', semua fieldnya immutable (tidak bisa diubah sebagian).
    user1.login_count += 1;
    println!("User {} sudah login {} kali.", user1.username, user1.login_count);


    // 5. Struct Update Syntax (Copy Data Cepat)
    // Mau bikin user2 yang datanya MIRIP user1, cuma beda email?
    // Pakai syntax '..user1' di baris terakhir.
    let user2 = AkunZeroize {
        email: String::from("bot@zeroize.com"),
        username: String::from("bot_01"),
        ..user1 // Sisanya (active, login_count) nyontek dari user1
    };
    
    println!("User 2 active? {}", user2.active);


    // 6. Menggunakan Tuple Struct
    let hitam = Warna(0, 0, 0);
    println!("Nilai Merah: {}", hitam.0); // Akses pakai index seperti tuple

    
    // 7. Pemanggilan Struct dengan implementasi
    let p = Persegi::new(10); // Panggil static method pakai ::
    println!("Luas: {}", p.hitung_luas()); // Panggil method pakai .
}