use std::io;

fn main() {
    let numero:f64 = coletor();
    let resultado:Notas = troco(numero);
    println!("NOTAS:");
    println!("{} nota(s) de R$ 100.00", resultado.n100);
    println!("{} nota(s) de R$ 50.00", resultado.n50);
    println!("{} nota(s) de R$ 20.00", resultado.n20);
    println!("{} nota(s) de R$ 10.00", resultado.n10);
    println!("{} nota(s) de R$ 5.00", resultado.n5);
    println!("{} nota(s) de R$ 2.00", resultado.n2);
    println!("MOEDAS:");
    println!("{} moeda(s) de R$ 1.00", resultado.m1);
    println!("{} moeda(s) de R$ 0.50", resultado.m050);
    println!("{} moeda(s) de R$ 0.25", resultado.m025);
    println!("{} moeda(s) de R$ 0.10", resultado.m010);
    println!("{} moeda(s) de R$ 0.05", resultado.m005);
    println!("{} moeda(s) de R$ 0.01", resultado.m001);
}

fn coletor() -> f64 {
    let mut number:String = String::new();
    io::stdin().read_line(&mut number).expect("msg");
    return number.trim().parse::<f64>().expect("msg");
}

fn troco(numero:f64) -> Notas{
    let n100:i32 = (numero / 100 as f64) as i32; let mut temp:f64 = numero % 100 as f64;
    let n50:i32 =    (temp / 50    as f64) as i32; temp = temp % 50   as f64;
    let n20:i32 =    (temp / 20    as f64) as i32; temp = temp % 20   as f64;
    let n10:i32 =    (temp / 10    as f64) as i32; temp = temp % 10   as f64;
    let n5:i32 =     (temp / 5     as f64) as i32; temp = temp % 5    as f64;
    let n2:i32 =     (temp / 2     as f64) as i32; temp = temp % 2    as f64;
    let m1:i32 =     (temp / 1     as f64) as i32; temp = temp % 1    as f64;
    let m050:i32 =   (temp / 0.50  as f64) as i32; temp = temp % 0.50 as f64;
    let m025:i32 =   (temp / 0.25  as f64) as i32; temp = temp % 0.25 as f64;
    let m010:i32 =   (temp / 0.10  as f64) as i32; temp = temp % 0.10 as f64;
    let m005:i32 =   (temp / 0.05  as f64) as i32; temp = temp % 0.05 as f64; 
    let m001:i32 =   (temp / 0.01  as f64).round() as i32;   
    
    let data:Notas = Notas {
        n100: n100,
        n50: n50,
        n20: n20,
        n10: n10,
        n5: n5,
        n2: n2,
        m1:m1,
        m050: m050,
        m025: m025,
        m010: m010,
        m005: m005,
        m001: m001
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
    m1: i32,
    m050: i32,
    m025: i32,
    m010: i32,
    m005: i32,
    m001: i32
}