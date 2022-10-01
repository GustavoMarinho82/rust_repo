fn main () {

    let vetor = vec![1, 2, 3];
    
    let closure = move || println!("O valor de vetor ({:?}) foi capturado!", vetor);

//Essa linha causa um erro, porque o valor de vetor foi movido pro closure e porque Vec não implementa o trait de Cópia
    //println!("Valor do vetor: {:?}", vetor);

    closure();
} 