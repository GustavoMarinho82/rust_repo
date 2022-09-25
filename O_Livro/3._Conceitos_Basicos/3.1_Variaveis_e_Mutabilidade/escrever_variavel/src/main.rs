fn main() {
//IMUTÁVEIS
//Por padrão, as variáveis sao imutáveis
    let x = 1;

        println!("Primeiro valor de x: {x}");

//Variaveis imutáveis precisam ser substituidas para terem seu valor alterado, para isso isso use o "let" de novo
    let x = x + 1;
        println!("Segundo valor de x: {x}");


//MUTÁVEIS
    let mut y = 3;
        y = 4;

    println!("\nValor de y: {y}");

    
//CONSTANTES
//Variáveis constantes não conseguem ser alteradas de nenhum jeito e precisam ter seu tipo especificado (u32)
//Por convenção, constantes são formatadas com todas as letras em maiusculo e com _ nos espacos (como: const TWO_TIMES_THREE = 2 * 3)
    const HORAS_EM_UM_MES: u16 = 24*30;

    println!("\nValor da constante: {HORAS_EM_UM_MES}");
}