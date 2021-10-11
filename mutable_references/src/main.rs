fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    /* mutable references have one big restriction: you can have only one mutable reference to a
     * particular piece of data in a particular scope.
     * see: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references
     *
     * We also cannot have a mutable reference while we have an immutable one. Users of an
     * immutable reference don’t expect the values to suddenly change out from under them! However,
     * multiple immutable references are okay because no one who is just reading the data has the
     * ability to affect anyone else’s reading of the data
     *
     * Note that a reference’s scope starts from where it is introduced and continues through the
     * last time that reference is used. For instance, this code will compile because the last
     * usage of the immutable references occurs before the mutable reference is introduced
     */
    let r3 = &mut s;
    println!("{}", r3);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
