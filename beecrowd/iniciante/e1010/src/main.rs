use std::io;

fn main() {
    let a = coletar();
    let b = coletar();

    let total = (a.qnt as f32 * a.valor_uni) + (b.qnt as f32 * b.valor_uni);

    println!("VALOR A PAGAR: R$ {:.2}", total);
}

fn coletar()-> Numero {
    let mut str_number:String = String::new();
    io::stdin().read_line(&mut str_number).expect("msg");

    let teste: Vec<&str> = str_number.split(' ').collect();

    let data = Numero {
        cod: teste[0].trim().parse::<i32>().expect("msg"),
        qnt: teste[1].trim().parse::<i32>().expect("msg"),
        valor_uni: teste[2].trim().parse::<f32>().expect("msg")
    };
    return data;
}

struct Numero {
    cod: i32,
    qnt: i32,
    valor_uni: f32
}