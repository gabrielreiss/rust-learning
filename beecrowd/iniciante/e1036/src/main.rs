use std::io;
fn main() {
    let data:Coeficientes = coleta();
    bhaskara(data);
}

fn coleta() -> Coeficientes {
    let mut str_a:String = String::new();
    io::stdin().read_line(&mut str_a).expect("msg");
    let str_a:Vec<&str> = str_a.split(" ").collect();
    let data = Coeficientes {
        a: str_a[0].trim().parse::<f64>().expect("msg"),
        b: str_a[1].trim().parse::<f64>().expect("msg"),
        c: str_a[2].trim().parse::<f64>().expect("msg")
    };
    return data;
}

struct Coeficientes {
    a: f64,
    b: f64,
    c: f64
}

fn bhaskara(data:Coeficientes) -> String {
    let delta:f64 = data.b.powf(2.0) - 4.0 * data.a * data.c;
    if  delta < 0.0
    ||  data.a == 0.0 {
        println!("Impossivel calcular");
        return "Impossivel calcular".to_string();
    } else {
        let r1: f64 = (-data.b + delta.sqrt()) / (2.0 * data.a);
        let r2: f64 = (-data.b - delta.sqrt()) / (2.0 * data.a);
        println!("R1 = {:.5}", r1);
        println!("R2 = {:.5}", r2);
        return format!("R1 = {:.5} R2 = {:.5}", r1, r2).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t1(){
        let r = bhaskara(Coeficientes {a:10.0, b:20.1, c:5.1});
        assert_eq!(r, "R1 = -0.29788 R2 = -1.71212");
    }

    #[test]
    fn t2(){
        let r = bhaskara(Coeficientes {a:0.0, b:20.0, c:5.0});
        assert_eq!(r, "Impossivel calcular");
    }

    #[test]
    fn t3(){
        let r = bhaskara(Coeficientes {a:10.3, b:203.0, c:5.0});
        assert_eq!(r, "R1 = -0.02466 R2 = -19.68408");
    }

    #[test]
    fn t4(){
        let r = bhaskara(Coeficientes {a:10.0, b:3.0, c:5.0});
        assert_eq!(r, "Impossivel calcular");
    }
}