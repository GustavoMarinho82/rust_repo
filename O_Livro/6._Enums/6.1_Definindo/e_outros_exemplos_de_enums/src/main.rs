#![allow(dead_code)]

enum Ip {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum RedeSocial {
    Facebook,
    Twitter,
    Reddit,
    Whatsapp,
    //...
}

enum Pais {
    Brasil,
    //Único país que importa
}

enum MoedaAmerican {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
enum Mensagem {
    Sair,
    Mover { x: i32, y: i32 },   //Dados declarados como num struct
    Escrever(String),
    MudarCor(u8, u8, u8),       //Dados declarados como numa tupla
}
    impl Mensagem {/*METODOS*/}

fn main() {}