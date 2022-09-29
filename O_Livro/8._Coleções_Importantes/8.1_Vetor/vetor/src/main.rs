fn main() {
    
//Declaração de um vetor vazio
    let mut vazio: Vec<u8> = Vec::new();

    //Adicionando valor a um vetor (Para atualizar um vetor, ele precisa ser mutável)
        vazio.push(1);
        vazio.push(2);
        vazio.push(3);


//A macro 'vec!' permite criar um vetor com valores já setados
    let mut vetor = vec![2u8, 4, 6, 8];

    //Mas mesmo assim, o vetor ainda pode ser atualizado
        vetor.push(10);


//Chamando os valores de um vetor
    let primeiro = &vetor[0];

    println!("Vetor n° 0 via 'get': {:?}", vetor.get(0));                   //Printa Some(valor)
    println!("Vetor n° 0 via '&':   {} | {}", primeiro, &vetor[0]);        //Printa só o valor


//Printando os valores de um vetor via 'for'
    println!("\nFOR");

    for i in &vetor {
        println!("{}", i);
    }


//Usando 'match' para conferir se há um valor em uma determinada posição
    let centesimo = vetor.get(99);
        match centesimo {
            Some(valor) => println!("Centésimo valor: {}", valor),
            None => println!("Não há um centésimo valor"),
        }

}
