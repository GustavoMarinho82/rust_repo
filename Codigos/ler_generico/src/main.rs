fn main() {
    println!("Digite um ---");

    if let Ok(oi) = ler_input("".to_string()) {
        println!("Foi digitado: {}", oi);
    
    } else {
        println!("Um valor inválido foi digitado");
    }
}

fn ler_input<T: std::str::FromStr> (ex: T) -> Result<T, T::Err>{
    
    let mut t = String::new();
        std::io::stdin().read_line(&mut t).unwrap();
        let t = t.trim().to_string();

    match t.parse::<T>() {
        Ok(a) => return Ok(a),
        Err(e) => Err(e),
    }
}