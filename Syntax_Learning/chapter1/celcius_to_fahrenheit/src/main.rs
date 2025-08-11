use std::io; // Standart input/output kütüphanesini dahil ediyoruz

fn main() {
    // Kullanıcıdan Celsius değeri girmesini istiyoruz
    println!("Lütfen dönüştürmek istediğiniz Celsius değerini giriniz:");
    
    // Yeni bir String değişkeni oluşturuyoruz (mutable - değiştirilebilir)
    let mut celsius = String::new();
    
    // Standart girişten (klavye) veri okuyoruz
    io::stdin()
        .read_line(&mut celsius) // Okunan veriyi celsius değişkenine yazıyoruz
        .expect("Failed to read line"); // Hata durumunda program çöker ve bu mesajı gösterir
    
    // String'i f64 (64-bit floating point) sayıya dönüştürüyoruz
    // trim() - başındaki ve sonundaki boşlukları temizler
    // parse() - string'i sayıya çevirir
    let celsius: f64 = celsius.trim().parse().expect("Lütfen geçerli bir sayı giriniz!");
    
    // Celsius'u Fahrenheit'a çeviren fonksiyonu çağırıyoruz
    let fahrenheit = celsius_to_fahrenheit(celsius);
    
    // Sonucu ekrana yazdırıyoruz
    println!("{celsius}°C is {fahrenheit}°F");
}

// Celsius'tan Fahrenheit'a dönüşüm yapan fonksiyon
// celsius: f64 -> parametre olarak 64-bit float alır
// -> f64 -> 64-bit float döndürür
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    // Dönüşüm formülü: F = (C × 9/5) + 32
    (celsius * 9.0 / 5.0) + 32.0
}
