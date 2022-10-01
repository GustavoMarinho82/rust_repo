//Os closures são como funções rápidas e anônimas, que capturam o ambiente em que estão. Eles usam || em vez de ()

fn main() {

//É possível prender um closure dentro de uma variável
    let c_print = || println!("Um closure que printa essa frase");   

    let c_grande = || {
        
        let a = 1;
        let b = 2;
            println!("Soma de 'a' e 'b' = {}", a+b);
    };

    c_print();
    c_grande();

/* Os parametro de um closure ficam dentro do ||. Não é necessário especificar o tipo dos parâmetros, 
*   o compilador os seta com base nas vezes em que o closure é chamado
*/
    let c_com_parametro = |x| println!("\nValor inserido = {}", x);
    
        c_com_parametro(15u16); //seta o parâmetro x de 'c_com_parametro' como u16

/*As duas variáveis abaixo fazem exatamente a mesma coisa. 
*   Ou seja, não é necessário especificar o retorno de um closure
*/
    let c_com_retorno01 = || -> i32 { 7 + 8 };
    let c_com_retorno02 = || 7 + 8 ;

    println!("\nRetorno n° 1: {} \nRetorno n° 2: {}", c_com_retorno01(), c_com_retorno02());
}