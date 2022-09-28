struct Retangulo {
    base: u32,
    altura: u32,
}
/*Os metodos funcionam exatamente como uma função, só que eles são associados a um struct e ele podem receber uma 
*   entrada de chamada 'self', que representa o struct que chamou o metodo
*/
    impl Retangulo {
        fn area(&self) -> u32 {
           self.base * self.altura
        }

    //Um metodo pode receber mais de um parâmetro
        fn cabe_dentro(&self, outro: &Retangulo) -> bool {
            if (self.base < outro.base) && (self.altura < outro.altura) {
                return true
            } else {
                return false
            }
        }
    }

fn main() {
    let ret1 = Retangulo {
        base: 6,
        altura: 9,
    };
    let ret2 = Retangulo {
        base: 9,
        altura: 12,
    };

    println!("Areas dos Retangulos: \n ret1: {} \n ret2: {} \n", ret1.area(), ret2.area());

    println!("ret1 cabe dentro de ret2?: {}", ret1.cabe_dentro(&ret2));
    println!("ret2 cabe dentro de ret1?: {}", ret2.cabe_dentro(&ret1));
}
