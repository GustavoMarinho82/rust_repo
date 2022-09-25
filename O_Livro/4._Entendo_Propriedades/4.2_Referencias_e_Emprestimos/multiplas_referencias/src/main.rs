/*É possível ter várias referências imutáveis sobre uma variável ao mesmo tempo.
*  Mas, não é permitido ter duas ou mais ref. mutáveis ao mesmo tempo,
*  nem ter uma combinação de ref. mutáveis e imutáveis
*/

fn main() {
    let mut texto = String::from("Oi");

    let r1 = &texto;
    let r2 = &texto; //Se fosse "&mut texto", geraria um erro

    
    println!("{} {}", r1, r2); 
    /*A partir daqui, as ref. r1 e r2 foram eliminadas automaticamente pelo Rust,
    *  porque elas não foram usadas em nenhum outro lugar após o print
    *
    * Ou seja, uma ref. mutável já pode ser declarada
    */
    let r3 = &mut texto;

    println!("{}", r3);
    // r3 foi descartada
}