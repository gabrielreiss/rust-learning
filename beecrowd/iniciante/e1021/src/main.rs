use std::io;

//refatorando
fn main() {
    let notas:Vec<f64> = [100.0, 50.0, 20.0, 10.0, 5.0, 2.0].to_vec();
    let moedas:Vec<f64> = [1.0, 0.50, 0.25, 0.10, 0.05, 0.01].to_vec();
    let mut numero:f64 = coletor();

    println!("NOTAS:");
    for i in notas {numero = troco(numero, i);}
    println!("MOEDAS:");
    for i in moedas {numero = troco(numero, i);}
}

fn troco(numero:f64, moeda:f64) -> f64 {
    //gambiarra por causa de bug no R$ 0.01
    //agora tem um bug com R$ 0.02
    if numero > 0.01 {
        //codigo normal
        let qnt:i32 = (numero / moeda) as i32;
        let sobras:f64 = numero % moeda;
        print_resultado(numero, qnt, moeda);
        return sobras;
    } else if numero == 0.01 {
        let qnt = 1;
        let sobras:f64 = numero % moeda;
        print_resultado(numero, qnt, moeda);
        return sobras;
    } else {
        let qnt:i32 = (numero / moeda) as i32;
        let sobras:f64 = numero % moeda;
        print_resultado(numero, qnt, moeda);
        return sobras;
    }
}

fn print_resultado(numero:f64, qnt:i32, moeda:f64) {
    if numero > 1.0 {
        println!("{} nota(s) de R$ {:.2}", qnt, moeda);
    } else {
        println!("{} moeda(s) de R$ {:.2}", qnt, moeda);
    }
}

fn coletor() -> f64 {
    let mut number:String = String::new();
    io::stdin().read_line(&mut number).expect("msg");
    return number.trim().parse::<f64>().expect("msg");
}