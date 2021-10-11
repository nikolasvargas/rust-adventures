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
}
