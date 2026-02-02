// References & Borrowing

fn main() {
    // --- 1. Immutable Reference (&) ---
    let s1 = String::from("Kursi Gaming");

    // Kita meminjamkan s1 ke fungsi hitung_panjang pakai '&'
    let len = hitung_panjang(&s1); 

    // KARENA CUMA PINJAM: s1 MASIH BISA DIPAKAI DI SINI!
    println!("Panjang '{}' adalah {}.", s1, len);


    // --- 2. Mutable Reference (&mut) ---
    let mut s2 = String::from("Kursi");

    // Kita pinjamkan ke fungsi modif pakai '&mut' biar bisa diubah
    ubah_kursi(&mut s2);
    
    println!("Hasil modif: {}", s2); // Output: Kursi Gaming RGB
}

// Fungsi ini cuma "minjam lihat" (read-only)
fn hitung_panjang(s: &String) -> usize {
    s.len()
} // s keluar scope, tapi data asli TIDAK dihapus karena s cuma meminjam.

// Fungsi ini "minjam ubah" (read-write)
fn ubah_kursi(s: &mut String) {
    s.push_str(" Gaming RGB");
}