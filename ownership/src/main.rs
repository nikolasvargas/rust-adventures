fn main() {
    //The double colon (::) is an operator that allows us to namespace this particular "from" function
    //under the "String" type rather than using some sort of name like string_from.
    let _s = String::from("hello"); //stack alloc

    let mut s = String::from("hello"); //heap alloc

    s.push_str("w, world!"); // push_str() apprends a literal to a String

    println!("{}", s);

    //So... what's the difference here? Why can String be mutated but literals cannot? The
    //difference is how these two types deal with memory.
    //see: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#memory-and-allocation

    let mut x = 5;
    let y = x; // hmm... in this case y has a copy of X, not a reference;
    x = 20;
    println!("x={}, y={}", x, y);

    // Let's look at the String version:

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}", s2);
    // This looks very similar to the previous code, so we might assume that the way it works would
    // be the same. That is, the second line would make a copy of the value in s1 and bind it to s2.
    // But this isn't quite what happens.

    /*
    *
    *     ------------------       -------------
    *     |name     | value|       |index|value|
    *     |----------------|       -------------
    *     |ptr      |  ----------> |  0  |  h  |
    *     |----------------|       -------------
    *     |len      |  5   |       |  1  |  e  |
    *     |----------------|       |-----------|
    *     |capacity |  5   |       |  2  |  l  |
    *     ------------------       |-----------|
    *                              |  3  |  l  |
    *                              |-----------|
    *                              |  4  |  o  |
    *                              -------------
    *
    * A String is made up of three parts, shown on the left: a pointer to the memory that holds
    * the contents of the string, a length, and a capacity. This group of data is stored on the
    * stack. On the right is the memory on the heap that holds the contents.
    *
    * The length is how much memory, in bytes the contents of the String is currently using. The
    * capacity is the total amount of memory, in bytes, that the String has received from the
    * allocator. The difference between length and capacity matters, but not in this context.
    *
    * When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length,
    * and the capacity that are on the stack, We do not copy the data on the heap that the pointer
    * refers to. In other words, the data representation in memory looks like:
    *
    *
    *           s1
    *    ------------------
    *    |name     | value|
    *    |----------------|
    *    |ptr      |  --------
    *    |----------------|  |
    *    |len      |  5   |  |
    *    |----------------|  |    -------------
    *    |capacity |  5   |  |    |index|value|
    *    ------------------  |    -------------
    *                        ---> |  0  |  h  |
    *                        |    -------------
    *                        |    |  1  |  e  |
    *            s2          |    |-----------|
    *    ------------------  |    |  2  |  l  |
    *    |name     | value|  |    |-----------|
    *    |----------------|  |    |  3  |  l  |
    *    |ptr      |  --------    |-----------|
    *    |----------------|       |  4  |  o  |
    *    |len      |  5   |       -------------
    *    |----------------|
    *    |capacity |  5   |
    *    ------------------
    *
    *
    * The representation does not look like:
    *
    *           s1
    *    ------------------       -------------
    *    |name     | value|       |index|value|
    *    |----------------|       -------------
    *    |ptr      |  ----------> |  0  |  h  |
    *    |----------------|       -------------
    *    |len      |  5   |       |  1  |  e  |
    *    |----------------|       |-----------|
    *    |capacity |  5   |       |  2  |  l  |
    *    ------------------       |-----------|
    *                             |  3  |  l  |
    *                             |-----------|
    *                             |  4  |  o  |
    *                             -------------
    *
    *           s2
    *    ------------------       -------------
    *    |name     | value|       |index|value|
    *    |----------------|       -------------
    *    |ptr      |  ----------> |  0  |  h  |
    *    |----------------|       -------------
    *    |len      |  5   |       |  1  |  e  |
    *    |----------------|       |-----------|
    *    |capacity |  5   |       |  2  |  l  |
    *    ------------------       |-----------|
    *                             |  3  |  l  |
    *                             |-----------|
    *                             |  4  |  o  |
    *                             -------------
    *
    * That is what memory would look like if Rust instead copied the heap data as well.
    * If Rust did this, the operation s2 = s1 could be very expensive in terms of runtime
    * performance if the data on the heap were large.
    *
    *
    * To ensure memory safety, there's one more detail to what happens in this situation in Rust.
    * Instead of trying to copy the allocated memory, Rust considers s1 to no longer be valid and,
    * therefore, Rust doesn't need to free anything when s1 goes out of scope. Check out what
    * happens when you try to use s1 after s2 is created; it won't work:
    * ------------------------------------------------------------------
    *
    * let s1 = String::from("foo");
    * let s2 = s1;
    * println!("{}, bar", s1);
    *
    * ------------------------------------------------------------------
    * You'll get an error like this because Rust prevents you from using the invalidated
    * reference:
    *
    * error[E0382]: borrow of moved value: `s1`
    *  --> src/main.rs:5:28
    *   |
    *   |     let s1 = String::from("hello");
    *   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    *   |     let s2 = s1;
    *   |              -- value moved here
    *   |
    *   |     println!("{}, world!", s1);
    *   |                            ^^ value borrowed here after move
    *
    * error: aborting due to previous error
    *
    * For more information about this error, try `rustc --explain E0382`.
    * error: could not compile `ownership`
    *
    * To learn more, run the command again with --verbose.
    *
    * If you've heard the terms shallow copy and deep copy while working with other languages, the
    * concept of copying the pointer, length, and capacity without copying the data probably
    * sounds like making a shallow copy. But because Rust also invalidates the first variable,
    * instead of being called a shallow copy, it's known as a `move`.
    */

    // lets take look of ownership example working with functions.
    let my_string = String::from("some string"); // my_string comes to scope.
    takes_ownership(my_string); // my_string value moves into the function... and so is no longer valid here

    let my_number = 5; // my_number come into scope
    makes_copy(my_number); // my_number move into the function, but i32 is Copy, so it's okay to still use it afterward
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_string goes out of scope. Nothing special happens.
