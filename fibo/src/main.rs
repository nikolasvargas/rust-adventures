use std::io;

fn main() {
    println!("Fibo");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    let result: u32 = fibonnaci(number);
    println!("fibo of {} is: {}", number, result);
}

fn fibonnaci(number: u32) -> u32 {
    match number {
        0 => 0,
        1 | 2 => 1,
        _ => {
            fibonnaci(number - 1) + fibonnaci(number - 2)
        }
    }
}

