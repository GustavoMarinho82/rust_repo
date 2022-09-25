struct Pessoa {
	CPF: u64,
	nome: String,
	idade: u8,
	genero: char,
}

fn main() {

	let antonio = Pessoa {
		CPF: 12345678901,
		nome: String::from("Antonio Buzz Jorge"),
		idade: 47,
		genero: 'H',
	};

	println!("Nome: {}", antonio.nome);
    println!("CPF: {}", antonio.CPF);
    println!("Idade: {}", antonio.idade);
    println!("Genero: {}", antonio.genero);

	let copia_antonio = Pessoa {
		CPF: 10987654321,
        nome: String::from("Antoniu Jorgi"),
		//O "..antonio" manda completar as variáveis faltando com os mesmos valores de "antonio"
        ..antonio
	};

    println!("\nNome do Copiao: {}", copia_antonio.nome);
    println!("CPF do Copiao: {}", copia_antonio.CPF);
    println!("Idade do Copiao: {}", copia_antonio.idade);
    println!("Genero do Copiao: {}", copia_antonio.genero);

}