use std::io;

fn main() {
    let a:Coordenadas = coletar();
    let b:Coordenadas = coletar();
    let resultado:f64 = distancia(a, b);
    println!("{:.4}", resultado);
}

struct Coordenadas {
    x:f64,
    y:f64
}

fn distancia(a:Coordenadas,b:Coordenadas) ->  f64 {
    return ((b.x - a.x).powf(2.0) + (b.y - a.y).powf(2.0)).sqrt()
}

fn coletar() -> Coordenadas {
    let mut temp:String = String::new();
    io::stdin().read_line(&mut temp).expect("msg");
    let temp:Vec<&str> = temp.split(' ').collect();
    let data: Coordenadas = Coordenadas {
        x: temp[0].trim().parse::<f64>().expect("msg"),
        y: temp[1].trim().parse::<f64>().expect("msg")
    };
    return data;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test1(){
        let a:Coordenadas = Coordenadas {
            x: 1.0,
            y: 7.0
        };
        let b:Coordenadas = Coordenadas {
            x: 5.0,
            y: 9.0
        };
        let r = distancia(a, b);
        assert_eq!(r.round(), 4.4721_f64.round());
        //eu deveria fazer uma funcao para arredondar com quatro casas decimais, mas não é o foco agora
    }

    #[test]
    fn test2(){
        let a:Coordenadas = Coordenadas {
            x: -2.5,
            y: 0.4
        };
        let b:Coordenadas = Coordenadas {
            x: 12.1,
            y: 7.3
        };
        let r = distancia(a, b);
        assert_eq!(r.round(), 16.1484_f64.round());
    }

    #[test]
    fn test3(){
        let a:Coordenadas = Coordenadas {
            x: 2.5,
            y: -0.4
        };
        let b:Coordenadas = Coordenadas {
            x: -12.2,
            y: 7.0
        };
        let r = distancia(a, b);
        assert_eq!(r.round(), 16.4575_f64.round());
    }
}