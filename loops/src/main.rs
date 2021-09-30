fn main() {
    let mut counter = 0;
    //One of the uses of a loop is to retry an operation you know might fail, such as checking
    //whether a thread has completed its job. However, you might need to pass the result of that
    //operation to the rest of your code. To do this, you can add the value you want returned after
    //the break expression you use to stop the loop; that value will be returned out of the loop so
    //you can use it
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
    println!("Counter is??? {}", counter);
}
