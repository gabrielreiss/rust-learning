use std::io;
use std::convert::TryInto;

fn main() {
    let n: i32 = input();
    if 3 <= n && n <= 500 {
        let v: Vec<i32> = sequencia(n);
        let quantidade: i32 = regra_negocio(v);
        println!("{:?}", quantidade);
    } else {
        println!("ERRO: Quantidade fora do intervalo aceito.");
    }
}

fn input() -> i32 {
    let mut number:String = String::new();
    io::stdin().read_line(&mut number).expect("msg");
    return number.trim().parse::<i32>().expect("msg");
}

fn sequencia(n:i32) -> Vec<i32> {
    let mut vetor: Vec<i32> = Vec::new();

    while vetor.iter().count() <= (n -1).try_into().unwrap() {
        let number: i32 = input();
        let intervalo = vec![1,2];
        if intervalo.contains(&number) {
            vetor.push(number);
        }
    }
    return vetor;
}

fn regra_negocio(mut vetor:Vec<i32>) -> i32 {
    //Esse valor é a quantidade máxima de números da sequência
    //que poderiam ser marcados com um círculo, de modo que a
    //sequência de números marcados não contenha dois números
    //iguais consecutivos.

    let mut anterior: i32 = vetor[0];
    let mut contagem: i32 = 1;

    for i in &mut vetor[1..]{
        if *i != anterior {
            contagem += 1;
            anterior = *i;
        }
    }
    return contagem;
}