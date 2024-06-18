use std::io;

fn main() {
    let mut str_a = String::new();
    let mut str_b = String::new();

    io::stdin().read_line(&mut str_a).expect("Não foi possível ler o primeiro valor");
    io::stdin().read_line(&mut str_b).expect("Não foi possível ler o segundo valor");
    
    let a:i32 = str_a.trim().parse::<i32>().expect("Não foi possivel converter o primeiro valor");
    let b:i32 = str_b.trim().parse::<i32>().expect("Não foi possivel converter o segundo valor");

    let c:i32 = a * b; 
    println!("PROD = {}", c);
}