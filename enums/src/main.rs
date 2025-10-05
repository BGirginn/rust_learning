// 1) enum: yeni sayılabilir tür ilanı
// 2) TrafficLight: tür adı (PascalCase önerilir)
// 3) { ... }: varyant listesi, virgülle ayrılır
#[derive(Debug)] // 7) Debug: türü yazdırılabilir yapar
enum TrafficLight {
    Red,    // "verisiz" varyant (unit-like)
    Yellow, // verisiz
    Green,  // verisiz
}

fn main() {
    // 4) let: bağlama/değişken tanımı
    // 5) ::  : ad alanı ayracı (Tür::Varyant)
    let l1 = TrafficLight::Red;
    let l2: TrafficLight = TrafficLight::Green; // açık tip ek açıklaması

    // 6) println!: makro, ekrana yazdırır. "{}" yer tutucu
    println!("l1 {:?} ve l2 {:?} oluşturuldu", l1, l2);
}
