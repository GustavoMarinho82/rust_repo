use std::io::{self, Read}; 
use std::fs::File;

//Um erro de propagação é um erro que é trazido por uma função que não é a 'main'
fn main() {

//O erro propagado deve ser tratado na função 'main'
    match ler_arquivo() {
        Ok(texto) => println!("Texto do arquivo: \n{}", texto),
        Err(erro) => println!("Ocorreu um erro: \n> {}", erro),
    }
}

//Uma função que pode propraga um erro deve retornar um 'Result'
fn ler_arquivo() -> Result<String, io::Error> {

    let mut abrir_arq = match File::open("texto.txt") {               
        Ok(file) => file,
        Err(erro) => return Err(erro),
    };

    let mut arq = String::new();

        match abrir_arq.read_to_string(&mut arq) {
            Ok(_) => return Ok(arq),
            Err(erro) => return Err(erro),
        }
}