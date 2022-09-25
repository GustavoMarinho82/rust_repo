struct RGB(u8, u8, u8);
struct CoordenadasMine(i32, i32, i32);

fn main() {
    let vermelho = RGB(255, 0, 0);
    let ponto0 = CoordenadasMine(0, 0, 0);

    println!("RGB do vermelho: {}, {}, {}", vermelho.0, vermelho.1, vermelho.2);
}