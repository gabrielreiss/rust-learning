use std::io;
fn main() {
    let data:Pontos = coletor();

    if desigualdades(data.clone()) {
        println!("Perimetro = {:.1}", perimetro(data));
    } else {
        println!("Area = {:.1}", area_trapezio(data));
    }
}

fn coletor() -> Pontos {
    let mut str_a:String = String::new();
    io::stdin().read_line(&mut str_a).expect("msg");
    let str_a:Vec<&str> = str_a.split(" ").collect();
    let a:Vec<f32> = str_a.iter().map(|s:&&str| s.trim().parse::<f32>().expect("Não deu para converter")).collect();
    let data:Pontos = Pontos {
        a: a[0],
        b: a[1],
        c: a[2]
    };
    return data;
}

struct Pontos {
    a:f32,
    b:f32,
    c:f32
}

impl Clone for Pontos {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b.clone(),
            c: self.c.clone()
        }
    }
}

//desigualdade triangular
//ignora a soma dos angulos igual a 180
fn desigualdades(data:Pontos) -> bool {
    if data.a < (data.b + data.c)
    && data.b < (data.a + data.c)
    && data.c < (data.a + data.b) {
        return true;
    } else {
        return false;
    }
}

fn perimetro(data:Pontos) -> f32 {
    return data.a + data.b + data.c;
}

fn area_trapezio(data: Pontos) -> f32 {
    return (data.a + data.b) * data.c / 2.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let r = perimetro(Pontos {a:6.0, b: 4.0, c: 2.0});
        assert_eq!(r, 12.0);
    }

    #[test]
    fn t2() {
        let r = perimetro(Pontos {a:6.0, b: 4.0, c: 2.1});
        assert_eq!(r, 12.1);
    }

    #[test]
    fn t3() {
        let r = area_trapezio(Pontos {a:6.0, b: 4.0, c: 2.0});
        assert_eq!(r, 10.0);
    }

    #[test]
    fn t4() {
        let r = area_trapezio(Pontos {a:6.0, b: 4.0, c: 2.1});
        assert_eq!(r, 10.5);
    }
}