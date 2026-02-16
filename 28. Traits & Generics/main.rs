// Traits (Sifat/Kemampuan) & Generics (Tipe Umum)

/* 
   1. TRAITS: Mirip "Interface" di Java/PHP. 
      Mendefinisikan fungsionalitas yang bisa dimiliki oleh berbagai tipe data.
      Contoh bawaan: Debug (biar bisa diprint), Clone (biar bisa dicopy).

   2. GENERICS: Memungkinkan kita menulis fungsi/struct untuk "Tipe Apa Saja".
      Biasanya ditandai dengan `<T>`.
*/

// --- 1. MEMBUAT TRAIT (Definisi Kemampuan) ---
trait BisaTerbang {
    fn terbang(&self) -> String; // Hanya tanda tangan fungsi (signature)
    
    // Default implementation (opsional)
    fn mendarat(&self) {
        println!("Mendarat dengan gaya default...");
    }
}

// --- 2. STRUCT YANG AKAN PAKAI TRAIT ---
struct Burung {
    nama: String,
}

struct Pesawat {
    tipe: String,
}

// --- 3. IMPLEMENTASI TRAIT KE STRUCT ---

// Burung jadi punya kemampuan BisaTerbang
impl BisaTerbang for Burung {
    fn terbang(&self) -> String {
        format!("{} mengepakkan sayap!", self.nama)
    }
}

// Pesawat jadi punya kemampuan BisaTerbang
impl BisaTerbang for Pesawat {
    fn terbang(&self) -> String {
        format!("Pesawat {} menyalakan mesin jet!", self.tipe)
    }
    // Kita override (timpa) method default mendarat
    fn mendarat(&self) {
        println!("Pesawat {} menurunkan roda pendaratan.", self.tipe);
    }
}

fn main() {
    let pipit = Burung { nama: String::from("Pipit") };
    let boeing = Pesawat { tipe: String::from("Boeing 737") };

    // Panggil method dari trait
    println!("Burung: {}", pipit.terbang());
    pipit.mendarat(); // Pakai default

    println!("Pesawat: {}", boeing.terbang());
    boeing.mendarat(); // Pakai override

    // --- 4. GENERICS (Fungsi untuk Tipe Apa Saja) ---
    // Fungsi ini bisa menerima APAPUN asalkan punya trait BisaTerbang
    // T : BisaTerbang -> Artinya T haruslah tipe yang meng-implementasikan BisaTerbang
    fn pamer_terbang<T: BisaTerbang>(benda: T) {
        println!("Lihat aksi ini: {}", benda.terbang());
    }

    pamer_terbang(pipit);  // Bisa masukin Burung
    // pamer_terbang(boeing); // Bisa masukin Pesawat (Error jika boeing sudah dipindah ownershipnya, hati-hati!)
}