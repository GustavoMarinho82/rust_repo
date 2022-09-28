/*
* O Option é um enum da biblioteca padrão, ele substitui o tipo Null no Rust.
* Uma variável ser do tipo Option<> significa que pode ser tanto nada quanto algo.
*
* Esse é o Option enum:
enum Option<T> {
    None,
    Some(T),
}
*/

fn main() {
    
//Não é possível somar valores Option<>, mesmo que eles não sejam 'None'. Para isso é necessário converte-los antes.
    let n1: Option<i32> = Some(1);
    let n2: Option<i32> = Some(2);
    let n_nada: Option<i32> = None;

//Para associar um valor a um Option<>, é necessário que esse valor também seja do tipo Option<>
    let talvez: Option<String> = {
        println!("Digite algo: ");
        
        let mut t = String::new();
            std::io::stdin().read_line(&mut t).unwrap();
            t = t.trim().to_string();
        
        match t.as_str() {
            "" => None,
            _ => Some(t),
        }
    };

/*O enum Option<> não implementa o Display, fazendo necessário o uso de {:?}, 
*   o que irá printar Some(valor) ou None
*/
    println!("Numeros já setados:");
        println!("{:?}", n1);
        println!("{:?}", n2);
        println!("{:?}", n_nada);
    
    println!("\nTexto digitado: ");
        println!("{:?}", talvez);

}

