use std::io;
fn main() {
    let mut distancia:String = String::new();
    io::stdin().read_line(&mut distancia).expect("msg");
    let distancia:i64 = distancia.trim().parse::<i64>().expect("msg");
    let resultado:i64 = tempo(distancia);
    println!("{} minutos", resultado);
}

fn tempo(distancia:i64) -> i64 {
    let carro1:i32 = 60;
    let carro2:i32 = 90;
    let minutos:f64 = 60 as f64 / (carro2-carro1) as f64;
    return (distancia as f64 * minutos) as i64;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let r = tempo(30);
        assert_eq!(r, 60);
    }
    #[test]
    fn test2(){
        let r = tempo(110);
        assert_eq!(r, 220);
    }
    #[test]
    fn test3(){
        let r = tempo(7);
        assert_eq!(r, 14);
    }
}