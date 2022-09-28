/*Esse projeto não possui um recurso próprio do Rust, que no caso é um metodo especifico do struct, 
*   esse projeto utilizada uma função separada do struct. Veja o recurso em: 5.3_Metodo
*/
struct Retangulo {
    base: u32,
    altura: u32,
}

fn main() {
    let ret1 = Retangulo {
        base: 6,
        altura: 9,
    };

    println!("A area do ret1 é: {}", area(&ret1));
}

/*Nesse caso é recomendado usar uma referencia em vez da estrutura em si,
*   porque a função tá apenas consultando os valores, e não alterando eles
*/
fn area(t: &Retangulo) -> u32 {
    t.base * t.altura

}