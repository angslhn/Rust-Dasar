// Matematika

/* 
    Di Rust, operasi matematika dasar (+ - * / %) ada di level syntax.
    Tapi untuk operasi kompleks (akar, pangkat, trigonometri), kita menggunakan
    method yang melekat pada tipe data primitifnya (terutama f64 dan f32).
    
    Catatan: Sebagian besar method matematika bekerja optimal di tipe data Float (f64).
*/

fn main() {
    // --- 1. OPERASI PANGKAT & AKAR ---
    let angka: f64 = 16.0;

    // A. Akar Kuadrat (Square Root)
    // .sqrt() hanya ada di tipe float (f32/f64)
    let akar = angka.sqrt();
    println!("Akar dari 16 adalah: {}", akar);

    // B. Pangkat (Power)
    // .powf() = power float (pangkat desimal) -> 2.0 pangkat 3.5
    // .powi() = power integer (pangkat bulat) -> 2.0 pangkat 3 (lebih cepat)
    let basis: f64 = 2.0;
    println!("2 pangkat 3 (int)  : {}", basis.powi(3)); 
    println!("2 pangkat 3.5 (flt): {}", basis.powf(3.5)); 

    // --- 2. PEMBULATAN (Rounding) ---
    let pecahan: f64 = 4.7;
    let pecahan_neg: f64 = -4.7;

    println!("Asli: {}", pecahan);
    // Floor: Bulatkan ke Bawah
    println!("Floor (Lantai): {}", pecahan.floor()); 
    // Ceil: Bulatkan ke Atas
    println!("Ceil (Atap)   : {}", pecahan.ceil());  
    // Round: Bulatkan ke Terdekat (matematika umum)
    println!("Round (Dekat) : {}", pecahan.round()); 
    // Trunc: Buang komanya (ambil int saja)
    println!("Trunc (Potong): {}", pecahan.trunc());
    
    // --- 3. NILAI MUTLAK (Absolute) ---
    // Mengubah negatif jadi positif
    println!("Absolut dari -4.7: {}", pecahan_neg.abs());

    // --- 4. TRIGONOMETRI ---
    // Input untuk sin/cos/tan HARUS dalam satuan Radian, bukan Derajat.
    let sudut_derajat: f64 = 90.0;
    let radian = sudut_derajat.to_radians(); // Konversi dulu

    println!("Sin 90 derajat: {}", radian.sin());
    
    // --- 5. KONSTANTA MATEMATIKA ---
    // Menggunakan module std::f64::consts
    println!("Nilai PI: {}", std::f64::consts::PI);
    println!("Nilai Euler (e): {}", std::f64::consts::E);
}