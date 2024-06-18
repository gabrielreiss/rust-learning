use std::io;

fn main() {
    let mut str_a = String::new();
    let mut str_b = String::new();

    io::stdin().read_line(&mut str_a).expect("Não foi possível ler o primeiro valor");
    io::stdin().read_line(&mut str_b).expect("Não foi possível ler o segundo valor");
    
    let a:f64 = str_a.trim().parse::<f64>().expect("Não foi possivel converter o primeiro valor");
    let b:f64 = str_b.trim().parse::<f64>().expect("Não foi possivel converter o segundo valor");

    let c:f64 = ((a * 3.5) + (b * 7.5))/ 11.0; 
    println!("MEDIA = {:.5}", c);
}