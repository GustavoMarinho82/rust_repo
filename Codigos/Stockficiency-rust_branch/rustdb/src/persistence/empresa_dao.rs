use mysql::{*, prelude::*};

//CREATE
pub fn create(conn: &mut PooledConn) -> Result<()>{
    conn.exec_drop("INSERT INTO empresas (nome_empresa, email, senha) VALUES (:a, :b, :c)",
        params! {
            "a" => ler_input::<String>("Digite o nome: "),
            "b" => ler_input::<String>("Digite o email: "),
            "c" => ler_input::<String>("Digite a senha: ")
        })?;
    Ok(())
}

//READ
pub fn read(conn: &mut PooledConn) {
    let row: Vec<(u32, String, String, String)> = conn.query("SELECT id_empresa, nome_empresa, email, senha FROM empresas")
        .unwrap();

    for r in row {
        println!("ID: {} | NOME: {} | EMAIL: {} | SENHA: {}", r.0, r.1, r.2, r.3);
    }
}

//UPDATE
pub fn update(conn: &mut PooledConn) -> Result<()>{
    conn.exec_drop("UPDATE empresas SET nome_empresa=:b, email=:c, senha=:d WHERE id_empresa=:a", params! {
        "a" => ler_input::<u32>("Digite o ID da empresa"),
        "b" => ler_input::<String>("Digite o novo nome: "),
        "c" => ler_input::<String>("Digite o novo email: "),
        "d" => ler_input::<String>("Digite a nova senha: "),
        })?;
    Ok(())
}

//DELETE
pub fn delete(conn: &mut PooledConn) {
    conn.exec_drop("DELETE FROM empresas WHERE id_empresa=:a", params! {
        "a" => ler_input::<u32>("Digite o ID da empresa")
        }).unwrap();
}


//FUNCAO DE LEITURA
fn ler_input<T: std::str::FromStr>(texto: &str) -> Option<T> {

    println!("{}", texto);

    let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    
    if input.trim() == "".to_string() { return None };

    match input.trim().parse::<T>() {
        Ok(input) => Some(input),
        Err(_) => None
    }
}