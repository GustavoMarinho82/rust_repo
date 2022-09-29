fn main() {
    
    let numero = Some(12u8);

/*Essas duas partes servem para fazer exatamente o mesmo propósito, só que 
*   o 'if let' é menor e não possui a verificação do match, que é desnecessária nesse caso.
*/
    match numero {
        Some(n) => println!("O numero é: {}", n),
        _ => (),
    }

//Se o número for 'Some', 'n' é declarada com o mesmo valor dentro do 'Some' do 'numero'.
    if let Some(n) = numero {
        println!("O número é: {}", n)
    }
}
