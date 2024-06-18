use std::io;
use std::cmp::Ordering;
extern crate rand;
use rand::Rng;

fn main() {
    println!("Advinhe o numero!");

    let number_rand: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Insira um numero inteiro entre 1 e 100: ");

        let mut str_numero = String::new();

        io::stdin().read_line(&mut str_numero).expect("Coloque um numero de verdade!");
        let numero:i32 = str_numero.trim().parse::<i32>().expect("Erro ao converter o numero");

        if numero == number_rand {
            println!("Parabéns, você acertou. Chance de 1% de acertar");
        } else {
            println!("Você errou... O número era: {number_rand}. Tudo bem, as chances de errar eram 99%");
        }

        match numero.cmp(&number_rand) {
            Ordering::Less => println!("Número baixo"),
            Ordering::Greater => println!("Número alto"),
            Ordering::Equal => {
                println!("Número certo");
                break;
            }
        }
    }
}