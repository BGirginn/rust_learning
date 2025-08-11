fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;   // let mut olarak yapmasaydık bu satırda hata alırdık çünkü x değişkeni immutable (değiştirilemez) olurdu.
    // println!("The value of x is: {x}");
    // mut olsa da olmasa da eğişkenlerin türü her daim sabit olur

    // let mut spaces = "   ";
    // spaces = spaces.len();  // burada spaces başta string iken sonra int olmaya kalkıyor.

    // let _spaces = "   ";
    // let _spaces = _spaces.len();  // spaces i tekrar tanımlayıp içine üst satırdaki spacesi koyduk


    // INTEGER TYPES
    // i8 8 bitlik işaret içeren tamsayı -128 to 127
    // u8 8 bitlik işaret içermeyen tamsayı 0 to 255

    // FLOATING POINT TYPES
    // f32 32 bitlik float
    // f64 64 bitlik float  default olanı budur
    // let x = 2.0; // f64
    // let y: f32 = 3.0; // f32

    // BOOLEAN TYPE
    // bool true or false

    // CHARACTER TYPE
    // char tek harften oluşan karakter, unicode desteği var

    // STRING TYPE
    // let mut text: &str = "merhaba";
// text = "dünya";        // ✅ Bu çalışır! (yeniden atama)
// text = "başka metin";  // ✅ Bu da çalışır!
// text = "test";         // ✅ Farklı &str'lara point edebilirsin
// AMA:
// text[0] = 'M';      // ❌ Hala hata! İçerik değiştirilemez

    // let mut text = String::from("merhaba");

// 1. Yeniden atama (tıpkı &str gibi)
// text = String::from("dünya");        // ✅ Bu çalışır!
// text = "başka metin".to_string();    // ✅ Bu da çalışır!
// text = String::from("test");         // ✅ Farklı String'lere point edebilirsin

// 2. İçerik değiştirme (&str'dan farklı olarak)
// text.push('!');                      // ✅ Karakter ekle
// text.push_str(" eklenen");          // ✅ String ekle
// text.pop();                         // ✅ Son karakteri sil
// text.clear();                       // ✅ Hepsini temizle
// text.replace_range(0..1, "T");      // ✅ Belirli kısmı değiştir
// Başlangıç: 'merhaba'
// push('!'): 'merhaba!'
// push_str(" eklenen"): 'merhaba! eklenen'
// pop(): 'merhaba! eklene'
// clear(): ''
// Yeni içerik: 'test'
// replace_range(0..1, "T"): 'Test'

    // TUPLES
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup; // destructuring
    // println!("The value of y is: {y}");
    // println!("The value of z is: {z}");
    // println!("The value of tup.0 is: {}", tup.0); // indeks ile erişim


    main2(); // main2 fonksiyonunu çağırıyoruz, bu fonksiyon main fonksiyonunun dışında tanımlandı.
    print_labeled_measurement(5, 'h');

        let y = {
        let x = 3;
        x + 1 // c++ daki return ifadesi gibi, son ifade otomatik olarak return edilir 
        // bu bloğun son değeri x+1 olur ve return gibi tanımlanacak yerde sonuna noktalı virgül falan koymazsın
    };
    println!("The value of y is: {y}");

    // plus_one fonksiyonu çağrılıyor ve 5 değeri parametre olarak gönderiliyor
    let x = plus_one(5);
    println!("The value of x is: {x}");

    // plus_one fonksiyonu: bir i32 parametre alır ve i32 döndürür
    // x parametresine 1 ekleyip sonucu döndürür (return anahtar kelimesi kullanmadan)
    fn plus_one(x: i32) -> i32 {
        x + 1  // son ifade noktalı virgül olmadan yazıldığı için otomatik return edilir
    }


    // loop {
    //     println!("again!");
    // }

    
        let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    for number in (1..4).rev()  // bu kodda 1'den 3'e kadar olan sayıları ters sırada döngüye alır ve ekrana yazdırır 
    {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

pub fn main2() {
    println!("Hello from main2!");
}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}