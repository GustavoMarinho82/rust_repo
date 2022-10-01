fn main() {
    println!("Digite um ---");

    let oi = match ler_input::<u8>() {
        Ok(valor) => valor,
        Err(_) => { 
            println!("Um valor inválido foi digitado!"); 
                0 
        }
    };

    println!("O valor da variável é: {}", oi);
}

fn ler_input<T: std::str::FromStr>() -> Result<T, T::Err>{

    let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Falha ao ler o input!");

    let input = input.trim().parse::<T>()?;
        Ok(input)
}