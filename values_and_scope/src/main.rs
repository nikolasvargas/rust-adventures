// Returning values can also transfer ownership.
fn main() {
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    let s2 = String::from("Bar"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); //s2 is moved into fn, which also moves its return value into s3
} // here, s1 and s3 goes out of scope and is dropped, s2 goes out of scope but was moved, so nothign happens;

// this fn will move its return value into the function that calls it.
fn gives_ownership() -> String {
    let s = String::from("Foo");
    s
}

// this fn will take a String an return one
fn takes_and_gives_back(some_string: String) -> String { // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

/*
 * The ownership of a variable follows the same pattern every time: assigning a value to another
 * variable moves it. When a variable that includes data on the heap goes out of scope, the value
 * will be cleaned up by drop unless the data has been moved to be owned by another variable.
 */
