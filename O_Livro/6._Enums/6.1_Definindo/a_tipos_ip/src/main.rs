#[derive(Debug)]

//Um enum é feito para enumerar os tipos relevantes que uma determinada coisa pode ser, como um IP, que pode ser do tipo 4 ou 6
enum IpTipo {
    V4,
    V6,
}

fn main() {
//Enums são tipos de dados, como u8 e String
    let velho: IpTipo = IpTipo::V4;
    let novo = IpTipo::V6;

    println!("Qual é o tipo de IP velho e novo?");
    println!(" velho: {:?}", velho);
    println!(" novo: {:?}", novo);
}
