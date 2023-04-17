use std::io;

fn main() {
    loop { 
        println!("\nCALCULADORA");

            println!("Digite um valor");
                let v1: f32 = ler_f32();
        
            println!("Digite outro valor");
                let v2: f32 = ler_f32();

        let opcao: u8 = ler_com_menu();

        match opcao {
            1 => println!("Resultado: {}", (v1 + v2)),
            2 => println!("Resultado: {}", (v1 - v2)),
            3 => println!("Resultado: {}", (v1 * v2)),
            4 => println!("Resultado: {}", (v1 / v2)),
            5 => break,
            _ => ()
        }
    }
}


fn ler_f32 () -> f32 {
    let mut temporario = String::new();

        io::stdin()
            .read_line(&mut temporario)
            .unwrap();
            
    temporario
        .trim()
        .parse()
        .unwrap()
}

fn ler_com_menu () -> u8 {
    println!("\nMENU");
        println!("1 - Adicao");
        println!("2 - Subtracao");
        println!("3 - Multiplicacao");
        println!("4 - Divisao");
        println!("5 - Fechar o programa");

    let mut temporario = String::new();

        io::stdin()
            .read_line(&mut temporario)
            .unwrap();
            
    temporario
        .trim()
        .parse()
        .unwrap()
}