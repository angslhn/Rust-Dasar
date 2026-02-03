// File I/O

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write}; // Trait Read & Write harus dibawa biar fungsi bacanya aktif

fn main() -> std::io::Result<()> {
    // KITA PAKAI '?' DI MAIN AGAR ERROR HANDLING RAPI
    // (Syarat: main harus return Result)

    // --- 1. MEMBUAT & MENULIS (Create) ---
    // Hati-hati: Ini akan MENGHAPUS isi lama jika file sudah ada!
    {
        let mut file = File::create("zeroize_db.txt")?;
        
        // Menulis byte string (b"...")
        file.write_all(b"Instagram: pass123\n")?; 
        
        // Atau pakai macro write! (lebih enak buat format string)
        write!(file, "Facebook: rahasia\n")?;
    } // File otomatis ditutup di sini (Scope berakhir)


    // --- 2. MENAMBAHKAN DATA (Append) ---
    // Kalau pakai File::create, data lama hilang.
    // Kalau mau nambah, pakai 'OpenOptions'.
    {
        let mut file = OpenOptions::new()
            .write(true) // Mode tulis
            .append(true) // Mode tambah di akhir
            .open("zeroize_db.txt")?;

        writeln!(file, "Twitter: burungbiru")?; // writeln! otomatis nambah \n
    }


    // --- 3. MEMBACA FILE (Read) ---
    {
        let mut file = File::open("zeroize_db.txt")?;
        let mut isi_file = String::new();
        
        // Baca semua isi file dan masukkan ke variabel string
        file.read_to_string(&mut isi_file)?;

        println!("\n=== ISI DATABASE ===");
        println!("{}", isi_file);
    }

    Ok(())
}