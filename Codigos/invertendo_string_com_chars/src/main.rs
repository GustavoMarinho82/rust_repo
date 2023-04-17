fn main() {
    let frase = "abcde";
        let letras = frase.chars();
    
    println!("PRIMEIRO FOR"); 
    
    //É usado a funcao .clone() para que a variavel não seja descartada após o for
    for letra in letras.clone() {
        println!("{}", letra);
    }

    println!("SEGUNDO FOR");

    //.rev() inverte a direção do iterador
    for letra in letras.rev().clone() {
        println!("{}", letra);
    }

    //.collect::<T> transforma o iterador em uma coleção do tipo T
    let frase_invertida = frase.chars().rev().collect::<String>();
    println!("Frase invertida: {}", frase_invertida);
}
