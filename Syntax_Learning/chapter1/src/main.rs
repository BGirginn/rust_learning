use std::io;  // rustun kendi içinde bulunan kütüphanelerde std yazarız.
use std::cmp::Ordering; // Ordering, karşılaştırma sonuçlarını temsil eden bir enum'dur
use rand::Rng; // Rng trait'i rastgele sayılar üretmek için kullanılır

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();  // mut olmazsa o değişekene ilk atadığın değer değişmez mesela sayi=5; dersen sayi=10 hata verir
    // mut kullandığın değişkenlere sonradan yeni değer atayabilirsin ama tür değişikliği yapamazsın


    io::stdin()
        .read_line(&mut guess) // read_line fonksiyonu, kullanıcının girdiği veriyi alır ve guess değişkenine atar
        .expect("Failed to read line"); // expect, eğer read_line başarısız olursa programı durdurur ve hata mesajı gösterir

    let guess: u32 = guess.trim().parse().expect("Please enter a number!");

    println!("You guessed: {guess}"); // {degisken adi} kullanımı, değişkenin değerini yazdırmak için kullanılır
    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is: {secret_number}");

    match guess.cmp(&secret_number) { // her türlü karşılaştırma için kullanıyoruz
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

// &mut, değişkenin referansını alır ve bu referans üzerinden değişiklik yapar. burada bahsedilen referans, değişkenin kendisi değil, onun bellekteki adresidir.


// cargo new <project_name> ile proje oluşturulur.
