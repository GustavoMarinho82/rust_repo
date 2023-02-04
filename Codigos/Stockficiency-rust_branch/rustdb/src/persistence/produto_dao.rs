use mysql::{*, prelude::*};

//CREATE
pub fn create(conn: &mut PooledConn) -> Result<()>{    
    conn.exec_drop("INSERT INTO produtos (codigo_produto, nome_produto, qtd_estoque, preco) VALUES (:a, :b, :c, :d)",
        params! {
            "a" => ler_input::<String>("Digite o código: "),
            "b" => ler_input::<String>("Digite o nome: "),
            "c" => ler_input::<u32>("Digite a quantidade em estoque: "),
            "d" => ler_input::<f64>("Digite o preço: ")
        })?;
    Ok(())
}

//READ
pub fn read(conn: &mut PooledConn) {
    let row: Vec<(String, String, u32, f64)> = conn.query("SELECT codigo_produto, nome_produto, qtd_estoque, preco FROM produtos")
        .unwrap();

    for r in row {
        println!("COD: {} | NOME: {} | Qtd: {} | PREÇO: {}", r.0, r.1, r.2, r.3);
    }
}

//UPDATE
pub fn update(conn: &mut PooledConn) -> Result<()>{
    conn.exec_drop("UPDATE produtos SET nome_produto=:b, qtd_estoque=:c, preco=:d WHERE codigo_produto=:a", params! {
        "a" => ler_input::<String>("Digite o código do produto"),
        "b" => ler_input::<String>("Digite o novo nome: "),
        "c" => ler_input::<u32>("Digite a quantidade em estoque: "),
        "d" => ler_input::<f64>("Digite o preço: ")
        })?;
    Ok(())
}

//DELETE
pub fn delete(conn: &mut PooledConn){
    conn.exec_drop("DELETE FROM produtos WHERE codigo_produto=:a", params! {
        "a" => ler_input::<String>("Digite o código do produto")
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