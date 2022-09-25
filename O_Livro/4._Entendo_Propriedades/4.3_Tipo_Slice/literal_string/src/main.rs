/*Não leve os cometários desse código como verdades absolutas, eles vieram de uma interpretação 
*   de um texto que não tentou explicar diretamente o assunto que comentários se referem
*/

fn main() {
    //O "literal" é uma ref, porque ela aponta pra onde tá o valor de "Olá" dentro do código
    let literal = "Olá";

    //Já o "texto" armazena o valor de "Olá", por meio do "String::from()"
    let texto = String::from("Olá");

    println!("{} {}", literal, texto);
}
