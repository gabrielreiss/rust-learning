use std::io;
fn main() {
    let n:Vec<i32> = coletor();
    let mut n_sort = n.clone();
    n_sort.sort();

    for num in n_sort.iter() {
        println!("{}", num);
    }

    println!("");
    
    for num in n.iter() {
        println!("{}", num);
    }
}

fn coletor() -> Vec<i32> {
    let mut str_a:String = String::new();
    io::stdin().read_line(&mut str_a).expect("msg");
    let numeros:Vec<&str> = str_a.split(" ").collect();
    //let numeros:Vec<i32> = numeros.iter().map(|s| s.parse::<i32>().unwrap()).collect();
    let numeros_int:Vec<i32> = numeros.iter().map(|s:&&str| s.trim().parse::<i32>().expect("Não deu para converter")).collect();
    return numeros_int;
}