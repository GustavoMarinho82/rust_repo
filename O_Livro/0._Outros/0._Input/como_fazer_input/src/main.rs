use std::io;    //Crate (ou biblioteca) usada para fazer o input

fn main() {

    println!("Digite uma palavra");

    //Declaração de uma variável tipo String vazia (String::new() = vazio)
        let mut texto = String::new();

        //Chama uma função da crate
           io::stdin()
            //Lê a linha digitada e atribui o valor digitado a uma variável
                .read_line(&mut texto)
            //Caso essa função retorne um erro, escreve o que está entre aspas (apesar de escrever muito texto além disso)
                .expect("Falha ao ler a linha!");


    println!("Agora, digite um numero");

        let mut numero = String::new();

            io::stdin()
                .read_line(&mut numero)  
                .expect("Falha ao ler a linha!");
        
    //Nao eh possivel um numero ser lido pela funcao acima, entao sera necessario converter uma String lida para um numero
    //Normalmente nao eh necessario o ".trim()", mas no caso de uma String lida via input eh necessario. O .trim() elimina "/n" e "/r/n" presentes na string
        let numero: i32 = numero.trim().parse().expect("Um numero inteiro não foi digitado!");


    println!("\nPalavra: {}Numero: {}", texto, numero);

}