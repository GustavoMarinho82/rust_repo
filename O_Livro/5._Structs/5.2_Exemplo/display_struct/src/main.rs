#[derive(Debug)]    //Exige que seja possível imprimir informações de depuração 
struct Triangulo {
    base: u32,
    altura: u32,
}

fn main() {
    let tri = Triangulo {
        base: 6,
        altura: 9,
    };

    println!("tri =");
        println!("  {:?}", tri);  //{:?} especifica que o formato de saída a ser usado é o Debug
        println!("  OU");
        println!("  {:#?}", tri); //{:#?} versão alternativa ao {:?}

    dbg!(&tri);  //imprime o arquivo e a linha onde tá o dbg, junto com o valor resultante da expressão. E retorna a propriedade do valor

println!("\n-------------------------------------------------------------------------\n");
    
    let n1 = 5;

    let tri2 = Triangulo {
        base: dbg!(6*n1),   //imprime a expressão (6*n1) e seu resultado
        altura: 9,
    };

    dbg!(&tri2);
}