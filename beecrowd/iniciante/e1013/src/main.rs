use std::io;
fn main() {
    let mut numeros_str:String = String::new();
    io::stdin().read_line(&mut numeros_str).expect("msg");
    let numero:Vec<&str> = numeros_str.split(' ').collect();
    let data = Numero {
        a: numero[0].trim().parse::<i64>().expect("msg"),
        b: numero[1].trim().parse::<i64>().expect("msg"),
        c: numero[2].trim().parse::<i64>().expect("msg")
    };
    let temp:i64 = maior(data.a, data.b);
    let resultado:i64 = maior(temp, data.c);

    println!("{} eh o maior", resultado);
}

fn maior(a: i64, b: i64) -> i64 {
    return (a+b+(a-b).abs())/2;
}

struct Numero {
    a: i64,
    b: i64,
    c: i64
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn teste_1(){
        let r = maior(7, 14);
        assert_eq!(r, 14);
    }
    #[test]
    fn teste_2(){
        let r = maior(14, 106);
        assert_eq!(r, 106);
    }
    #[test]
    fn teste_3(){
        let r = maior(217, 14);
        assert_eq!(r, 217);
    }
    #[test]
    fn teste_4(){
        let r = maior(14, 6);
        assert_eq!(r, 14);
    }
}