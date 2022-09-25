//Crates (ou bibliotecas) importadas
use rand::Rng;          //Para usar uma crate não padrão será necessário por-la no Cargo.toml junto com a sua versão. std indica que é uma crate padrão do Rust
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Chute um número!");

    let numero_secreto = rand::thread_rng().gen_range(1..=100);  //Usa uma funcao da crate rand para gerar um numero aleatorio entre 1 e 100

    loop {                                          //O loop termina quando o "break" for executado
        println!("Por favor digite seu chute.");

        let mut chute = String::new();      //String::new() atribui um valor de uma string vazia. Para ocorrer um input é necessário uma string vazia

        io::stdin()
            .read_line(&mut chute)  //Lê a linha e atribui a o valor uma variável. (O & indica referência a uma variável) 
            .expect("Falha ao ler a linha");     //Caso essa função retorne um erro, escreve o que está entre aspas

        let chute: u32 = match chute.trim().parse() {   //O match é usado para decidir o que fazer em seguida com base na comparação entre oq eu vem antes do {} e o {}
            Ok(num) => num,
            Err(_) => continue,     //Se o input der um erro, o programa só continuará a ser executado
        };

        println!("Você chutou: {chute}");

        match chute.cmp(&numero_secreto) {
            Ordering::Less => println!("Pequeno de mais!"),
            Ordering::Greater => println!("Grande de mais!"),
            Ordering::Equal => {
                println!("Você ganhou!");
                break;
            }
        }
    }
}