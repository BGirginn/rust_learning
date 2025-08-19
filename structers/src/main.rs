struct User {
    active: bool,
    username: String,
    sign_in_count: u64,
}

fn main() {

// alttakine tuple deniyor. bunun structure den farkı okunabilirliği düşük ve kullanımı zor ama ufak gruplar için daha hızlı bir çözüm
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
    };

    // Alanlara erişim
    println!("username = {}", user1.username);
    println!("active   = {}", user1.active);
    println!("sign in count = {}", user1.sign_in_count);

}


