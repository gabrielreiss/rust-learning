use std::io;

pub fn main() {

    let a:i32 = number();
    let b:i32 = number();
    let c:i32 = number();
    let d:i32 = number();

    let diff:i32 = (a * b) - (c * d);

    println!("DIFERENCA = {}", diff);
}

fn number() -> i32{
    let mut str_a = String::new();
    io::stdin().read_line(&mut str_a).expect("Não foi possível ler o primeiro valor");
    let a:i32 = str_a.trim().parse::<i32>().expect("Não foi possivel converter o primeiro valor");
    return a;
}