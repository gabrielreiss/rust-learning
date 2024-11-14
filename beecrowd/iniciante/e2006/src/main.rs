use std::io;
use std::str::FromStr;

fn main() {
    let mut _input_string:String = String::new();
    
    io::stdin().read_line(&mut _input_string).expect("ERRO: not int");
    let cha_resposta:i32 = _input_string.trim().parse().unwrap();
    _input_string.clear();

    io::stdin().read_line(&mut _input_string).expect("ERRO: not string");
    let numbers: Vec<i32> = _input_string.split_whitespace()
                                         .map(|part| {
                                            i32::from_str(part).unwrap()})
                                         .collect();

    let contagem = numbers.iter().filter(|&x| *x == cha_resposta).count();
    println!("{}", contagem);
}
