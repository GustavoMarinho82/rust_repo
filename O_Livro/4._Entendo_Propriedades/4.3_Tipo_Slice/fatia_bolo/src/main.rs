fn main() {
    let texto = String::from("Bolo de cenouro");

    //slice é um tipo de ref. que referencia apenas uma parte do valor de uma variável
    let fatia = &texto[0..4];   //também poderia ser [..4]

    println!("{}", fatia);

    
    //Também é possível ter slices de não-strings
    let n: [u8; 5]= [2, 4, 6, 8, 10];

    let fatia2 = &n[3..];

    let mut x=0;
    while x!= fatia2.len() {
        println!("{}", fatia2[x]);
        x+=1;
    }
}
