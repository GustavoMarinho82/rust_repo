struct Triangulo {
    base: u32,
    altura: u32,
}

fn main() {
    let tri1 = Triangulo {
        base: 6,
        altura: 9,
    };

    print!("A area do tri1 é: {}", area_tri(&tri1));
}

/*Nesse caso é recomendado usar uma referencia em vez da estrutura em si,
*   porque a função tá apenas consultando os valores, e não alterando eles
*/
fn area_tri(t: &Triangulo) -> u32 {
    (t.base * t.altura)/2

}