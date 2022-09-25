fn main() {
    let mut x: usize = 0;

    let tup: (u8, f32, char) = (1, 2.1, 'c');

    //println!("{}", tup.a);   //É impossível usar uma variável para chamar uma coordenada de uma tupla

        println!("Coordenada 1 tupla: {}", tup.0);
        println!("Coordenada 2 tupla: {}", tup.1);
        println!("Coordenada 3 tupla: {}", tup.2);

        
//Um array só armazena valores do mesmo tipo, diferente da tupla; o array possui um tamanho fixo
    let array: [u8; 5] = [5, 4, 3, 2, 1];

       for element in array {
            x +=1;

            //print!("\nCoordenada {} array: {element}", x);    //Pode ser feito dessa forma

            print!("\nCoordenada {x} array: {}", array[x-1]); //Para ler o [x-1], a variável x precisar ser usize (porque o 1 também é usize)   IMPORTANTISSIMO!!!
        }

}