use std::fmt;

//i32 para evitar algumas conversões de tipos
pub const MINUTOS_NUMA_HORA: i32 = 60;
pub const MINUTOS_NUM_DIA: i32 = 1440;

//PartialEq foi usado para ser possivel dizer se dois relógios são iguais
#[derive(PartialEq)]
pub struct Relogio {
    hora: u8,
    minuto: u8
}

impl Relogio {
    pub fn novo(horas: i32, minutos: i32) -> Self {
        let tempo_em_minutos: i32 = (horas*MINUTOS_NUMA_HORA) + minutos;
        let tempo_reduzido: i32 = tempo_em_minutos.rem_euclid(MINUTOS_NUM_DIA); //x.rem_euclid(y) = ((x % y) + y) % y
        
        return Relogio {
            hora: (tempo_reduzido / MINUTOS_NUMA_HORA) as u8,
            minuto: (tempo_reduzido % MINUTOS_NUMA_HORA) as u8
        }
    }

    pub fn add_minutos(&self, minutos: i32) -> Self {
        let tempo_em_minutos: i32 = ((self.hora as i32)*MINUTOS_NUMA_HORA) + (self.minuto as i32) + minutos;

        return Relogio::novo(0, tempo_em_minutos)
    }
}

//Display para implementar o .to_string()
impl fmt::Display for Relogio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //{:0>2} vai adicionar zeros na esquerda até o número ter tamanho de dois digitos
        write!(f, "{:0>2}:{:0>2}", self.hora, self.minuto) 
    }
}


fn main() {
    let r1 = Relogio::novo(23, -1439);
    let r2 = Relogio::novo(-10, 30);

    println!("r1 -> {}", r1.to_string());
    println!("r2 -> {}", r2.to_string());

    let r1 = r1.add_minutos(929);

    println!("r1 v2 -> {} \n", r1.to_string());

    if r1 == r2 {
        println!("r1 v2 é igual a r2 \n")
    }
}