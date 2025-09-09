enum Islem{
        Toplama(i32,i32),
        Metin(&'static str),
        Sifirla,
    }

fn islem_yap(islem:Islem) {
    match islem {
        Islem::Toplama(a, b) => println!("Toplam: {}", a + b),
        Islem::Metin(metin) => println!("Metin: {}", metin),
        Islem::Sifirla => println!("Sıfırlandı!"),
    }
    
}