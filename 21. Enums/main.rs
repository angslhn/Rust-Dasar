// Enums

// 1. Enum Biasa (Classic)
// Cuma sekadar label pilihan.
enum Role {
    Admin,
    User,
    Guest,
}

// 2. Enum Super (Menyimpan Data)
// Setiap varian bisa punya tipe data yang BEDA-BEDA.
enum Pesan {
    Quit,                       // Tidak bawa data
    Move { x: i32, y: i32 },    // Bawa data seperti Struct (Named fields)
    Write(String),              // Bawa data String (Tuple style)
    ChangeColor(i32, i32, i32), // Bawa 3 integer (RGB)
}

// 3. Method di dalam Enum
// Sama seperti Struct, Enum juga bisa punya fungsi (impl).
impl Pesan {
    fn call(&self) {
        println!("Pesan diterima!");
    }
}

fn main() {
    // Membuat instance Enum
    let pesan1 = Pesan::Write(String::from("Halo Rust"));
    let pesan2 = Pesan::Move { x: 10, y: 20 };

    pesan1.call();

    // 4. Pattern Matching dengan Enum (The Power of Match)
    // Cara mengambil data dari dalam Enum adalah dengan 'match'.
    process_message(pesan1);
    process_message(pesan2);


    // 5. THE NULL KILLER: Option Enum
    // Rust TIDAK PUNYA 'null'. Gantinya adalah Enum bawaan bernama Option<T>.
    // enum Option<T> {
    //    Some(T), // Ada isinya
    //    None,    // Kosong (pengganti null)
    // }

    let ada_angka: Option<i32> = Some(5);
    let tidak_ada: Option<i32> = None;

    // Cara pakai data di dalam Option (harus di-handle manual)
    match ada_angka {
        Some(isi) => println!("Isinya adalah: {}", isi),
        None => println!("Data kosong!"),
    }
}

fn process_message(msg: Pesan) {
    match msg {
        Pesan::Quit => {
            println!("Keluar program...");
        }
        Pesan::Write(teks) => {
            println!("Pesan masuk: {}", teks);
        }
        Pesan::Move { x, y } => {
            println!("Bergerak ke koordinat: {}, {}", x, y);
        }
        Pesan::ChangeColor(r, g, b) => {
            println!("Ganti warna ke RGB: {}, {}, {}", r, g, b);
        }
    }
}