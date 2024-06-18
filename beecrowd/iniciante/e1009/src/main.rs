use std::io;

fn main() {
    let mut name:String = String::new();
    let mut fixo_str:String = String::new();
    let mut vendas_str:String = String::new();

    io::stdin().read_line(&mut name).expect("msg");
    io::stdin().read_line(&mut fixo_str).expect("msg");
    io::stdin().read_line(&mut vendas_str).expect("msg");

    let fixo:f32 = fixo_str.trim().parse::<f32>().expect("msg");
    let vendas:f32 = vendas_str.trim().parse::<f32>().expect("msg");

    let total:f32 = calculo(fixo, vendas);
    println!("TOTAL = R$ {:.2}", total);
}

fn calculo(fixo:f32, vendas:f32)->f32 {
    return round(fixo + vendas * 0.15,2);
}

fn round(x:f32, decimals:u32) -> f32 {
    let y:f32 = 10i32.pow(decimals) as f32;
    return (x * y).round() / y
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn calculo1(){
        let result = calculo(500.00,1230.30);
        assert_eq!(result, 684.55);
    }

    #[test]
    fn calculo2(){
        let result = calculo(700.00,0.00);
        assert_eq!(result, 700.00);
    }

    #[test]
    fn calculo3(){
        let result = calculo(1700.00,1230.50);
        assert_eq!(result, 1884.58);
    }
}