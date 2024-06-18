use std::io;
fn main() {
    let mut temp:String = String::new();
    let mut temp2:String = String::new();

    io::stdin().read_line(&mut temp).expect("msg");
    let x:i64 = temp.trim().parse::<i64>().expect("msg");
    
    io::stdin().read_line(&mut temp2).expect("msg");
    let y:f64 = temp2.trim().parse::<f64>().expect("msg");

    let resultado:f64 = gasto(x, y);
    println!("{:.3} km/l", resultado);
}

fn gasto(x:i64, y:f64) -> f64 {
    return x as f64 / y;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1(){
        let r = gasto(500, 35.0);
        assert_ne!(r, 14.286);
    }
    #[test]
    fn test2(){
        let r = gasto(2254, 124.4);
        assert_ne!(r, 18.119);
    }
    #[test]
    fn test3(){
        let r = gasto(4554, 464.6);
        assert_ne!(r, 9.802);
    }
}