//https://learning-rust.github.io/docs/e4.unwrap_and_expect.html

fn main() {

    let algo: Option<u8> = Some(5);
    let nada: Option<u8> = None;

//Se o valor anterior ao .unwrap_or() for 'None' ou 'Err', o valor dentro dos () é passado no lugar
    assert_eq!(5, nada.unwrap_or(5));

//O unwrap_or_else() funciona da mesma forma, só que ele exige que um closure seja passado
    assert_eq!(5, nada.unwrap_or_else(|| if let Some(x) = algo {x} else {0}));
    assert_eq!(5, nada.unwrap_or_else(|| 5));

//O mesmo para unwrap_or_default(), só que ele passa o valor padrão do tipo de dado
    assert_eq!(0, nada.unwrap_or_default());


    println!("Se você está lendo quer dizer que nenhum assert_eq retornou um erro!")
}
