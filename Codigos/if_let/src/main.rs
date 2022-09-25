fn main() {
    let nome: Option<&str> = Some("Pereira");

    if let Some(a) = nome {
        println!("Olá, {}", a)
    }
}
