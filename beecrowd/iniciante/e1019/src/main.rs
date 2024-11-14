use std::io;

fn main() {
    println!("{}", converte(input()))
}

fn input() -> i32 {
    let mut number:String = String::new();
    io::stdin().read_line(&mut number).expect("ERRO: not string compativel");
    return number.trim().parse::<i32>().expect("ERRO: erro ao converter para número");
}

fn converte(number:i32) -> String {
    let horas:i32       = number / (60 * 60);
    let resto_horas:i32 = number % (60 * 60);
    let minutos:i32     = resto_horas / 60;
    let segundos:i32    = number % 60;
    return format!("{}:{}:{}", horas, minutos, segundos)
}
