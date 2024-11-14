use std::io;

fn main() {
    let mut terceiro: i32 = input();
    terceiro *= 4;
    println!("{}", terceiro);
}

fn input() -> i32 {
    let mut number:String = String::new();
    io::stdin().read_line(&mut number).expect("ERRO");
    return number.trim().parse::<i32>().expect("ERRO");
}

