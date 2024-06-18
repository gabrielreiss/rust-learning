use std::io;

fn main() {
    let numeros = coletar();
    println!("TRIANGULO: {:.3}", triangulo(numeros.a, numeros.c));
    println!("CIRCULO: {:.3}", circulo(numeros.c));
    println!("TRAPEZIO: {:.3}", trapezio(numeros.a, numeros.b, numeros.c));
    println!("QUADRADO: {:.3}", quadrado(numeros.b));
    println!("RETANGULO: {:.3}", retangulo(numeros.a, numeros.b));
}

fn coletar() -> Numero {
    let mut str_number:String = String::new();
    io::stdin().read_line(&mut str_number).expect("msg");
    let temp: Vec<&str> = str_number.split(' ').collect();

    let data = Numero {
        a: temp[0].trim().parse::<f64>().expect("msg"),
        b: temp[1].trim().parse::<f64>().expect("msg"),
        c: temp[2].trim().parse::<f64>().expect("msg")
    };

    return data;
}

struct Numero {
    a: f64,
    b: f64,
    c: f64
}

fn triangulo(a:f64,c:f64) -> f64{
    return a * c / 2.0;
}

fn circulo(c: f64) -> f64{
    return 3.14159 * c.powf(2 as f64);
}

fn trapezio(a:f64,b:f64,c:f64) -> f64 {
    return (a + b) * c / 2.0;
}

fn quadrado(b: f64) -> f64 {
    return b *b;
}

fn retangulo(a:f64, b:f64) -> f64 {
    return a * b;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn triangulo1(){
        let r = triangulo(3.0, 5.2);
        assert_eq!(r, 7.8);
    }

    #[test]
    fn triangulo2(){
        let r = triangulo(12.7, 15.2);
        assert_eq!(r, 96.52);
    }

    #[test]
    fn circulo1(){
        let r = circulo(5.2);
        assert_eq!(r, 84.949);
    }

    #[test]
    fn circulo2(){
        let r = circulo(15.2);
        assert_eq!(r, 725.833);
    }

    //fazer os demais testes
}