use std::io;
fn main() {
    let numeros = coleta(); 
    verificacao(numeros);
}

fn coleta() -> Data {
    let mut str_a:String = String::new();
    io::stdin().read_line(&mut str_a).expect("msg");
    let str_a:Vec<&str> = str_a.split(' ').collect();
    let data = Data {
        a: str_a[0].trim().parse::<i64>().expect("msg"),
        b: str_a[1].trim().parse::<i64>().expect("msg"),
        c: str_a[2].trim().parse::<i64>().expect("msg"),
        d: str_a[3].trim().parse::<i64>().expect("msg")
   };
   return data;
}

struct Data {
    a:i64,
    b:i64,
    c:i64,
    d:i64
}

fn verificacao(data: Data) -> &'static str {
    if data.b > data.c
    && data.d > data.a
    && (data.c + data.d) > (data.a + data.b)
    && data.c >=0
    && data.d >=0
    && data.a % 2 == 0 {
        println!("Valores aceitos");
        return "Valores nao aceitos";
    } else {
        println!("Valores nao aceitos");
        return "Valores nao aceitos";
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn t1(){
        let r = verificacao(Data {a:5, b:6, c:7, d:8});
        assert_eq!(r, "Valores nao aceitos");
    }

    #[test]
    fn t2(){
        let r = verificacao(Data {a:2, b:3, c:2, d:6});
        assert_eq!(r, "Valores aceitos");
    }
}