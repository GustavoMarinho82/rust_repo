#![allow(dead_code)]
//Saia da caixinha!!!
enum Mensagem {
    Sair,
    Mover { x: i32, y: i32 },   //Dados declarados como num struct
    Escrever(String),
    MudarCor(u8, u8, u8),       //Dados declarados como numa tupla
}
//Enums também podem ter metodos
    impl Mensagem {
        fn executar(&self) {
            match self {
                Self::Sair => {
                    println!("Saindo!");
                },

                Self::Mover {x, y} => {
                    println!("Movendo para as coordenadas: {}, {}", x, y);
                },

                Self::Escrever(texto) => {
                    println!("Escrevendo: {}", texto)
                },

                Self::MudarCor(x, y, z) => {
                    println!("Mudando as cores para o RGB({}-{}-{})", x, y, z)
                },
            }
        }
    }

fn main() {

    let sms = Mensagem::Escrever(String::from("Olá"));
        sms.executar();

    let transporte = Mensagem::Mover{x: 10, y: 25};
        transporte.executar();

    let cor = Mensagem::MudarCor(255, 255, 255);
        cor.executar();

    let quitar = Mensagem::Sair;
        quitar.executar();
}