// Owenership -> Stack and Heap
// Referrence and borrowing
// Slices
fn main() {

    //&str ile string arasındaki fark String e atanan yazı push vesaire ile dinamik olarak değiştirilebilirken &str sabittir ve değiştirilemez.
    // &str de değişkene veri atanmış olmalı ancak String ile dışarıdan veri alınımı yapılabilir. &str ile input yapılamaz
    let s1 = String::from("hello"); // s1 sahibi

    let s2 = s1; // sahiplik s2'ye geçti (move)

    // println!("{}", s1); // ❌ hata: s1 artık sahibi değil
    println!("{}", s2); // ✔ s2 yazdırılabilir

    let s1 = String::from("hello");
    let s2 = s1.clone(); // heap verisini kopyalar

    println!("{}", s1); // ✔ s1 hâlâ geçerli
    println!("{}", s2); // ✔ s2 de geçerli




let s = String::from("hello");
    takes_ownership(s); // sahiplik fonksiyona geçti

    // println!("{}", s); // ❌ hata: s artık geçerli değil

fn takes_ownership(some_string: String) {
    println!("Aldım: {}", some_string);
} // some_string scope'tan çıkınca drop 



    
    // 1) String oluşturuyoruz
    let mut s = String::from("hello");

    // 2) Sadece okuyan fonksiyonu çağırıyoruz (immutable borrow)
    read_string(&s); // sahiplik bende, sadece okuma izni verdim

    // 3) İçeriği değiştiren fonksiyonu çağırıyoruz (mutable borrow)
    change_string(&mut s); // sahiplik bende, ama içerik değiştirilebilir

    // 4) Değişmiş halini yazdırıyoruz
    println!("Son hali: {}", s);
}

// Sadece okuma izni veren fonksiyon
fn read_string(s: &String) {
    println!("Okudum: {}", s);
    // s.push_str("!"); // ❌ hata: &String değiştirilemez
}

// İçeriği değiştirme izni veren fonksiyon
fn change_string(s: &mut String) {
    s.push_str(", world"); // String'in sonuna ekleme yap
    println!("Eklenen: {}", s);
}
