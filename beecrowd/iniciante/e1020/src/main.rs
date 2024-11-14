use std::io;

fn main() {
    let idade = converte(input());
    println!("{} ano(s)", idade.ano);
    println!("{} mes(es)", idade.mes);
    println!("{} dia(s)", idade.dia);
}

fn input() -> i32 {
    let mut idade:String = String::new();
    io::stdin().read_line(&mut idade).expect("ERROR: read string");
    return idade.trim().parse::<i32>().expect("ERROR: parse to int");
}

struct Tempo {
    ano: i32,
    mes: i32,
    dia: i32
}

fn converte(idade:i32) -> Tempo {
    return Tempo {
        ano: idade / 365,
        mes: (idade % 365) / 30,
        dia: (idade % 365) % 30
    }
}