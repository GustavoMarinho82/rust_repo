pub fn print_user_route() {
    crate::models::user_model::print_user_model();  //A função de user_model foi importada aqui
                                                    //por meio de "crate", que faz referencia ao src (raiz)
    println!("user_route");
}

/*
//FORMA MAIS PRÁTICA E BONITA DE FAZER ISSO

use crate::models::user_model::print_user_model;

pub fn print_user_route() {
  print_user_model();
  println!("user_route");
}

*/


//TAMBÉM É POSSÍVEL RENOMEAR UMA FUNÇÃO COM O USE
/*

use crate::models::user_model::print_user_model as novo_nome;

pub fn print_user_route() {
  novo_nome();
  println!("user_route");
}

*/