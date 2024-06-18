use std::io;

fn main() {
    
    let mut str_a = String::new();
    let mut str_b = String::new();

    io::stdin().read_line(&mut str_a).expect("Failed to read line a");
    io::stdin().read_line(&mut str_b).expect("Failed to read line b");

    let a: i32 = str_a.trim().parse().expect("Invalid input a: not an integer");
    let b: i32 = str_b.trim().parse().expect("Invalid input b: not an integer");
    let c: i32 = a + b;

    let text = format!("X = {}", c);
    println!("{}", text);
}
