/*
* O Result é um enum da biblioteca padrão, assim como o Option.
* Ele é dividido em Ok e Err, onde um indica que tudo ocorreu como deveria e o outro que houve um erro
*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

//Esse código foi feito sem o uso de closures

use std::{fs::File, io::ErrorKind};

fn main() {
    let abrir_arq_resultado = File::open("texto.txt");

    let _abrir_arq = match abrir_arq_resultado {         
        
        Ok(file) => file,
        Err(erro) => match erro.kind() {
        
        //Tetando analisar o tipo de Erro, se for do tipo NotFound, executa:
            ErrorKind::NotFound => match File::create("texto.txt") {          
                Ok(arq_criado) => arq_criado,
                Err(erro) => panic!("Falha ao criar o arquivo: {}", erro)
            },

        //Se não for do tipo NotFound, executa:
            outro_tipo => panic!("Erro: {}", outro_tipo),
        }
    };
}

/* SEM COMENTÁRIOS NO MEIO

let _abrir_arq = match abrir_arq_resultado {
      
        Ok(file) => file,
        Err(erro) => match erro.kind() {
        
            ErrorKind::NotFound => match File::create("texto.txt") {                
                Ok(arq_criado) => arq_criado,
                Err(erro) => panic!("Falha ao criar o arquivo: {}", erro)
            },

            outro_tipo => panic!("Erro: {}", outro_tipo),
*/