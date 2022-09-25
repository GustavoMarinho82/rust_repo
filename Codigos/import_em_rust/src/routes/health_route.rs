pub fn print_health_route() {
    super::user_route::print_user_route();  //super faz referência ao escopo pai, que nesse caso é routes
    // Igual à crate::routers::user_route::print_user_route();
    println!("health_route");
}