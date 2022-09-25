use std::io;

fn main() {

    loop { 

        println!("\nCALCULADORA");

            println!("Digite um valor");
                let v1: i64 = ler_i64();
        
            println!("Digite outro valor");
                let v2: i64 = ler_i64();

        println!("\nMENU");
            println!("1 - Adicao");
            println!("2 - Subtracao");
            println!("3 - Multiplicacao");
            println!("4 - Divisao");
            println!("5 - Fechar o programa");
            
            let opcao: u8 = ler_u8();


        if opcao == 1 {
            println!("Resultado: {}", (v1 + v2));

        } else if opcao == 2 {
            println!("Resultado: {}", (v1 - v2));

        } else if opcao == 3 {
            println!("Resultado: {}", (v1 * v2));

        } else if opcao == 4 {
            println!("Resultado: {}", (v1 / v2));

        } else if opcao == 5 {
            break;
        }

    }
}


fn ler_i64 () -> i64 {
    let mut temporario = String::new();

        io::stdin()
            .read_line(&mut temporario)
            .unwrap();
            
    temporario
        .trim()
        .parse()
        .unwrap()
}

fn ler_u8 () -> u8 {
    let mut temporario = String::new();

        io::stdin()
            .read_line(&mut temporario)
            .unwrap();
            
    temporario
        .trim()
        .parse()
        .unwrap()
}