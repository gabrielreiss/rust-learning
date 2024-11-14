use std::io;

fn main() {
    let pressao_desejada: i32 = input();
    let pressao_lida:i32 = input();

    let diferenca: i32 = pressao_desejada - pressao_lida;
    println!("{}", diferenca);
}

fn input() -> i32 {
    let mut number:String = String::new();
    io::stdin().read_line(&mut number).expect("msg");
    return number.trim().parse::<i32>().expect("msg");
}
