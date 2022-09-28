#[derive(Debug)]
//Ao invés de criar um struct só para unir um tipo de enum à um dado, é possível atribuir outros tipos de dados aos tipos de um enum
enum IpTipo {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let velho: IpTipo = IpTipo::V4(127, 0, 0, 1);
    let novo = IpTipo::V6(String::from("::1"));

    println!("LOOPBACK");
    println!(" Em IP{:?}", velho);
    println!(" Em IP{:?}", novo);
}
