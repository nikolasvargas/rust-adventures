fn main() {
    let s1 = String::from("hello");
    let len = calc_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calc_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.

/* Note that we pass &s1 into calc_length and, in its definition, we take &String rather than
 * String. These ampersands are `references`, and they allow you to refer to some value without
 * taking ownership of it.
 *
 *          s                     s1
 *  ------------------     ------------------     -------------
 *  |name     | value|     |name     | value|     |index|value|
 *  |----------------|     |----------------|     |-----------|
 *  |ptr      |  --------->|ptr      |  --------->|  0  |  h  |
 *  |----------------|     |----------------|     |-----------|
 *                         |len      |  5   |     |  1  |  e  |
 *                         |----------------|     |-----------|
 *                         |capacity |  5   |     |  2  |  l  |
 *                         ------------------     |-----------|
 *                                                |  3  |  l  |
 *                                                |-----------|
 *                                                |  4  |  o  |
 *                                                -------------
 */
