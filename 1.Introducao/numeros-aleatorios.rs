use std::io;
extern crate rand;
use rand::Rng;

fn main() {
    println!("Advinhe o numero!");

    let number_rand: i32 = rand::thread_rng().gen_range(1..=100);

    println!("Insira um numero inteiro entre 1 e 100: ");

    let mut str_numero = String::new();

    io::stdin().read_line(&mut str_numero).expect("Coloque um numero de verdade!");
    let numero:i32 = str_numero.trim().parse::<i32>().expect("Erro ao converter o numero");

    if numero == number_rand {
        println!("Parabéns, você acertou. Chance de 1% de acertar");
    } else {
        println!("Você errou, tudo bem, as chances de errar eram 99%");
    }
}