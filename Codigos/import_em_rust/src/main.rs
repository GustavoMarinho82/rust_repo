//https://www.sheshbabu.com/posts/rust-module-system/

//Para importações feitas no mesmo no mesmo diretório de main, só é necesário um mod
mod config; 

/*Caso contrário, é necessário fazer o seguinte:
*   1- Criar uma pasta com um arquivo "mod.rs" ou, simplesmente, um .rs (ambos devem estar no mesmo lugar do main)
*   2- Fazer um mod na opção escolhido do arquivo que você queria importar de início
*/
mod routes; //o compilador irá procurar por routes.rs (no mesmo dir do main) ou routes/mod.rs

mod models;

fn main() {

    println!("main");

    config::print_config();

    routes::health_route::print_health_route(); //Exemplo de super
    routes::user_route::print_user_route();     //Exemplo de crate e use

}