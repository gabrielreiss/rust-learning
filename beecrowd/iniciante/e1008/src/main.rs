use std::io;

fn main() {
    
    let mut cad_str: String = String::new();
    let mut horas_str: String = String::new();
    let mut custo_str: String = String::new();
    
    io::stdin().read_line(&mut cad_str).expect("Insira um numero valido para o numero do funcionário");
    io::stdin().read_line(&mut horas_str).expect("Insira um numero valido para o numero de horas do funcionário");
    io::stdin().read_line(&mut custo_str).expect("Insira um numero valido para o valor por hora do funcionário");

    let cad:i32 = cad_str.trim().parse::<i32>().expect("Erro ao conveter o numero do funcionário para int");
    let horas:f32 = horas_str.trim().parse::<f32>().expect("Erro ao conveter o numero de horas do funcionário para int");
    let custo:f32 = custo_str.trim().parse::<f32>().expect("Erro ao conveter o valor por hora do funcionário para float");

    let salario:f32 = sal(horas, custo);

    println!("NUMBER = {cad}");
    println!("SALARY = U$ {salario}");
}

fn sal(horas:f32, custo:f32)-> f32 {
    return horas * custo;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sal1(){
        let result: f32 = sal(100.0,5.50);
        assert_eq!(result, 550.00);
    }

    #[test]
    fn test_sal2(){
        let result: f32 = sal(200.0,20.50);
        assert_eq!(result, 4100.00);
    }

    #[test]
    fn test_sal3(){
        let result: f32 = sal(145.0,15.55);
        assert_eq!(result, 2254.75);
    }
}