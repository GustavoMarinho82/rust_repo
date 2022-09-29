use std::io::{self, Read}; 
use std::fs::File;

fn main() {
    match ler_arquivo() {
        Ok(texto) => println!("Texto do arquivo: \n{}", texto),
        Err(erro) => println!("Ocorreu um erro: \n> {}", erro),
    }
}

/*O '?' funciona exatamente como os matches anteriores.
*   Se a expressão funcionar adequadamente, ela retorna o valor dentro de Ok,
*   senão, ela executa um equilavente ao 'Err(e) => return e', que serve para toda a função.
*
*   O '?' não pode ser usado na função 'main'
*/
fn ler_arquivo() -> Result<String, io::Error> {
    let mut abrir_arq = File::open("texto.txt")?;
    
    let mut arq = String::new();
        abrir_arq.read_to_string(&mut arq)?;
        
    //Esse Ok() é o retornado se a função chegar execução até aqui
        Ok(arq)
}