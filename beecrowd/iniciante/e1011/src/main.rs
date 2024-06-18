use std::io;

fn main() {
    let mut str_name:String = String::new();
    io::stdin().read_line(&mut str_name).expect("msg");
    let raio:i64 = str_name.trim().parse::<i64>().expect("msg");
    let volume:f64 = esfera(raio);
    println!("VOLUME = {:.3}", volume);
}

fn esfera(r:i64) -> f64 {
    let pi:f64 = 3.14159;
    return 4.0/3.0 * pi * r.pow(3) as f64;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculo1() {
        let result = esfera(3);
        assert_eq!(result.round(), 113.0);
    }

    #[test]
    fn calculo2() {
        let result = esfera(15);
        assert_eq!(result.round(), 14137.0);
    }

    #[test]
    fn calculo3() {
        let result = esfera(1523);
        assert_eq!(result.round(), 14797486502.0);
    }
        
}