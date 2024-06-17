use std::io;

fn main() {
    let pi: f64 = 3.14159;

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Deu erro na entrada dos dados");

    let resultado: f64 = input.trim().parse::<f64>().expect("Não deu para converter em float").powf(2.0) * pi;

    println!("A={:.4}", resultado);
}