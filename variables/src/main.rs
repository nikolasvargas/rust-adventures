fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // throw cannot assing twice to immutable variable
    // x = 6;
    // println!("The value of x is: {}", x);

    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    //shadowing
    let x = "override";

    println!("The value of y is: {}", x);
    let lol_cat = '😹';
    println!("{}", lol_cat);

    //tuples!!
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //patter matching to destructure
    let (x, y, z) = tup;

    println!("Values! {} - {} - {}", x, y, z);

    //access a tuple element directly by using a period... nice
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("nice :) {} - {} - {}", five_hundred, six_point_four, one);

    //every element of an array must have the same type.
    //Arrays in Rust are different from arrays is some other languages
    //because arrays in Rust have a fixed length, like tuples.
    //an array isn't as flexible as the vector type, though.
    let _arr = [1, 2, 3, 4, 5];

    //remember -> arrays in Rust have fixed sizes;
    let array_in_rust: [i32; 5] = [1, 2, 3, 4, 5];
    let _first = array_in_rust[0];
    let _second = array_in_rust[1];

    //Writing an array’s type this way looks similar to an alternative syntax for initializing an
    //array: if you want to create an array that contains the same value for each element, you can
    //specify the initial value, followed by a semicolon, and then the length of the array in
    //square brackets, as shown here
    let _a = [3; 5]; //equals to let a = [3, 3, 3, 3, 3];
}
