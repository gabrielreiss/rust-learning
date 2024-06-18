use std::io;

fn main() {
    let mut str_a = String::new();
    let mut str_b = String::new();
    let mut str_c = String::new();

    io::stdin().read_line(&mut str_a).expect("Não foi possível ler o primeiro valor");
    io::stdin().read_line(&mut str_b).expect("Não foi possível ler o segundo valor");
    io::stdin().read_line(&mut str_c).expect("Não foi possível ler o terceiro valor");
    
    let a:f64 = str_a.trim().parse::<f64>().expect("Não foi possivel converter o primeiro valor");
    let b:f64 = str_b.trim().parse::<f64>().expect("Não foi possivel converter o segundo valor");
    let c:f64 = str_c.trim().parse::<f64>().expect("Não foi possivel converter o segundo valor");

    let media:f64 = ((a * 2.0) + (b * 3.0) + (c * 5.0))/ 10.0; 
    println!("MEDIA = {:.1}", media);
}