#[derive(Debug)]
enum IpTipo {
    V4,
    V6,
}

//Structs podem implementar enums (enums são tipos de dados, como u8)
struct IP {
    tipo: IpTipo,
    endereco: String
}

fn main() {
    let lb_velho = IP {
        tipo: IpTipo::V4,
        endereco: String::from("127.0.0.1"),
    };

    let lb_novo = IP {
        tipo: IpTipo::V6,
        endereco: String::from("::1"),
    };

    println!("LOOPBACK");
    println!(" Em IP{:?} = {}", lb_velho.tipo, lb_velho.endereco);
    println!(" Em IP{:?} = {}", lb_novo.tipo, lb_novo.endereco);
}
