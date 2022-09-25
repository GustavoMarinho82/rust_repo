fn main() {
    let mut texto = String::from("Olá");

    mudar_emprestimo(&mut texto); //Envia uma referência mutável ao valor de "texto"

    println!("{} \n", texto);
}

//Essa função muda o emprestimo do valor de "texto", e devolve ele diferente
fn mudar_emprestimo(t: &mut String){
    t.push_str(", mundo!");

}   //Fim da vida da referência