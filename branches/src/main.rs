fn main() {
    let number = 6;

    //Using too many else if expressions can clutter your code, so if you have more than one, you
    //might want to refactor your code. Chapter 6 describes a powerful Rust branching construct
    //called match for these cases.
    if number % 4 == 0 {
        println!("it is divisible by 4");
    } else if number % 3 == 0 {
        println!("it is divisible by 3");
    } else if number % 2 == 0 {
        println!("it is divisible by 2");
    } else if number % 1 == 0 {
        println!("it is divisible by 1");
    } else {
        println!("nothing todo");
    }

    //Because if is an expression, we can use it on the right side of a let statement
    let condition = true;
    // For this works, both values must be of same type;
    // When we try to compile with types mismatched, we’ll get an error. The if and else arms have value types
    // that are incompatible, and Rust indicates exactly where to find the problem in the program
    // Rust needs to know at compile time what type the number variable is, definitively, so it can
    // verify at compile time that its type is valid everywhere we use number.
    let number = if condition { 5 } else { 6 };

    println!("number value: {}", number);
}
