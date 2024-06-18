use std::io;
fn main() {
    let tempo:i32 = coletar();
    let velocidade: i32 = coletar();
    let resultado:f32 = litros(tempo, velocidade);
    println!("{:.3}", resultado);
}

fn litros(tempo:i32, velocidade:i32) -> f32 {
    let eficiencia:i32 = 12;
    return (tempo *  velocidade) as f32 / eficiencia as f32;
}

fn coletar() -> i32 {
    let mut tempo:String = String::new();
    io::stdin().read_line(&mut tempo).expect("msg");
    let tempo:i32 = tempo.trim().parse::<i32>().expect("msg");
    return tempo;
}

fn round3(x:f32) -> f32 {
    return (x * 1000 as f32).round() / 1000 as f32;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let r = litros(10, 85);
        assert_eq!(round3(r), 70.833);
    }

    #[test]
    fn test2(){
        let r = litros(2, 92);
        assert_eq!(round3(r), 15.333);
    }

    #[test]
    fn test3(){
        let r = litros(22, 67);
        assert_eq!(round3(r), 122.833);
    }
}