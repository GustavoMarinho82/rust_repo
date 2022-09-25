use std::io;

fn main() {
    print_hello_world();

    print_valor(100);

    let x: i32 = ler_int();

    print!("^2 = {}", elevar_a_dois(x));
    
}

//FUNÇÃO SEM PARAMETROS E SEM RETORNO
fn print_hello_world() {
	println!("Hello World!")
}


//FUNÇÃO COM PARAMETROS E SEM RETORNO
fn print_valor(valor: i32) {
	println!("{}", valor);
}


//FUNÇÃO SEM PARAMETROS E COM RETORNO
fn ler_int() -> i32 {
    let mut temporario = String::new();

        io::stdin()
            .read_line(&mut temporario)
            .expect("Falha ao ler a linha!");
            
    temporario.trim().parse().expect("Falha ao converter a tipagem!")	//Esse valor será o retorno, pois ele não termina com ;     IMPORTANTISSIMO
}


//FUNÇÃO COM PARAMETROS E COM RETORNO
fn elevar_a_dois(valor: i32) -> i32 {
	valor * valor
}
