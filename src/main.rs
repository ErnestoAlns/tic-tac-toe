struct Matriz<T> {
    data: [[T; 3]; 3],
}

impl Matriz<i32> {
    pub fn identidad() -> Self {
        Matriz {
            data: [
                [1, 0, 0],
                [0, 1, 0],
                [0, 0, 1],
            ],
        }
    }

    pub fn imprimir(&self) {
        println!("Matriz Identidad 3x3:");
        for fila in &self.data {
            for elemento in fila {
                print!("{: <2}", elemento);
            }
            println!();
        }
    }
}

fn main() {
    let mi_matriz = Matriz::identidad();
    mi_matriz.imprimir();
}

