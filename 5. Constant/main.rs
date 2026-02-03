// Constants (Konstanta)

/* 
    Konstanta adalah nilai yang tidak boleh berubah sama sekali selama program berjalan.
    Berbeda dengan 'let', konstanta memiliki aturan yang lebih kaku.
*/

fn main() {
    // 1. Penulisan Wajib Uppercase
    // Sesuai konvensi Rust, konstanta harus menggunakan huruf kapital semua 
    // dan dipisahkan dengan underscore (_).
    const MAX_POINTS: u32 = 100_000; 

    // 2. Wajib Memberikan Tipe Data
    // Tidak seperti 'let', kita TIDAK BISA membiarkan Rust menebak tipenya.
    // Kita harus menuliskan ': tipe_data' secara eksplisit.
    const ID_SERVER: &str = "ID-01";

    // 3. Tidak Bisa Menggunakan 'mut'
    // Konstanta bukan sekadar variabel yang tidak bisa diubah (immutable),
    // tapi mereka SELAMANYA tidak bisa diubah. 'const mut' adalah error.

    // 4. Harus Berisi Nilai yang Pasti (Constant Expression)
    // Nilainya harus bisa dihitung saat kode di-compile, bukan saat program jalan.
    // Contoh:
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // VALID (bisa dihitung compiler)
    
    // let x = 5;
    // const DYNAMIC: i32 = x; // ERROR! Nilai x ditentukan saat program jalan (runtime).

    println!("Batas poin: {}", MAX_POINTS);
}

// 5. Bisa Diletakkan di Luar 'fn main()' (Global Scope)
// Berbeda dengan 'let' yang harus di dalam fungsi, 'const' bisa ditaruh di paling atas file
// agar bisa diakses oleh semua fungsi di file tersebut.
const APP_NAME: &str = "Maman Store";