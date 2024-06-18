use std::io;

fn main() {
    let numero:i32 = coletor();
    let resultado:Notas = troco(numero);
    println!("{}", numero);
    println!("{} nota(s) de R$ 100,00", resultado.n100);
    println!("{} nota(s) de R$ 50,00", resultado.n50);
    println!("{} nota(s) de R$ 20,00", resultado.n20);
    println!("{} nota(s) de R$ 10,00", resultado.n10);
    println!("{} nota(s) de R$ 5,00", resultado.n5);
    println!("{} nota(s) de R$ 2,00", resultado.n2);
    println!("{} nota(s) de R$ 1,00", resultado.n1);
}

fn coletor() -> i32 {
    let mut number:String = String::new();
    io::stdin().read_line(&mut number).expect("msg");
    return number.trim().parse::<i32>().expect("msg");
}

fn troco(numero:i32) -> Notas{
    let n100:i32 = numero / 100; let mut temp:i32 = numero % 100;
    let n50:i32 = temp / 50;   temp = temp % 50;
    let n20:i32 = temp / 20;   temp = temp % 20;
    let n10:i32 = temp / 10;   temp = temp % 10;
    let n5:i32 = temp / 5;    temp = temp % 5;
    let n2:i32 = temp / 2;    temp = temp % 2;
    let n1:i32 = temp / 1;
    
    let data:Notas = Notas {
        n100: n100,
        n50: n50,
        n20: n20,
        n10: n10,
        n5: n5,
        n2: n2,
        n1: n1
    };
    return data;
}
struct Notas {
    n100: i32,
    n50: i32,
    n20: i32,
    n10: i32,
    n5: i32,
    n2: i32,
    n1: i32
}