//! her bir derive kısmı her struct için bağımsız en başa hepsini tanımlayamazsın her birini kendi struct'ının üstüne yazmalısın
#[derive(Debug)] // bunu kullanarak structun println! ile yazdırılmasını sağlarsın   println!("rect1 is {rect1:?}"); seklinde
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Clone)]  // bu satır struct'ın clonelanabilir olmasını sağlar
struct User {
    active: bool,
    username: String,
    sign_in_count: u64,
    email: String,
}

fn main() {

// alttakine tuple deniyor. bunun structure den farkı okunabilirliği düşük ve kullanımı zor ama ufak gruplar için daha hızlı bir çözüm
// tupleda alanların adı yoktur sadece sırası ve tipi vardır
    let tup: (bool, i32, &str) = (true, 42, "alice");

    // index ile erişim
    println!("active = {}", tup.0); // true
    println!("count  = {}", tup.1); // 42
    println!("name   = {}", tup.2); // alice


// burada kullanılan ise struct ama önce class mantığı gibi en tepede template olan halini oluşturmak zorundasın
    // Struct örneği oluşturmak
    let user1 = User {
        active: true,
        username: String::from("alice"),
        sign_in_count: 5,
        email: String::from("alice@example.com"),
    };

    

    // Alanlara erişim
    println!("username = {}", user1.username);
    println!("active   = {}", user1.active);
    println!("sign in count = {}", user1.sign_in_count);
    println!("email = {}", user1.email);

    // let user2 = User {
    // active: user1.active,
    // username: user1.username,
    // email: String::from("another@example.com"),
    // sign_in_count: user1.sign_in_count,
    // };


    // büyük oranda aynı kalacak yeni bir structure oluşturuyorsan üsttekini değil alttakini kullan işleri hızlandırır
    let user2 = User {
    email: String::from("another@example.com"),
    ..user1.clone() // user1'in diğer alanlarını kopyalar
    };

    println!("user1.active = {}", user1.active);  // structure içindeki verilere buradaki user1.active gibi erişiriz
    println!("user2 email = {}", user2.email);


    // tuple struct da aynı tuple gibi ama alanların adı vardır 
    struct Color(i32, i32, i32);   // RGB için
    struct Point(i32, i32, i32);   // Nokta için

    let black = Color(0, 8, 0);   // siyah renk
    let origin = Point(0, 0, 15);  // koordinat orijini
    
    // erişim tuple gibi
    println!("R: {}", black.1);  // black.0 demek black tuplesinin 0. indexindeki eleman
    println!("X: {}", origin.2);


    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let scale = 2;
    let rect1 = Rectangle {
        width: 30 * scale,
        height: 50,
    };

    // 1️⃣ {:?} → kısa debug print
    println!("1) rect1 is {:?}", rect1);

    // 2️⃣ {:#?} → pretty debug print
    println!("2) rect1 is {:#?}", rect1);

    // 3️⃣ dbg! → dosya + satır + değer
    dbg!(&rect1);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

