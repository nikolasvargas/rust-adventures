//Function bodies are made up of a series of statements optionally ending in an expression. So far,
//we’ve only covered functions without an ending expression, but you have seen an expression as
//part of a statement. Because Rust is an expression-based language, this is an important
//distinction to understand. Other languages don’t have the same distinctions, so let’s look at
//what statements and expressions are and how their differences affect the bodies of functions.
//We’ve actually already used statements and expressions. Statements are instructions that perform
//some action and do not return a value. Expressions evaluate to a resulting value.

fn main() {
    let _x = 5;

    let y = {
        let x = 3;
        //Note the x + 1 line without a semicolon at the end, which is unlike most of the lines
        //you’ve seen so far. Expressions do not include ending semicolons. If you add a semicolon
        //to the end of an expression, you turn it into a statement, which will then not return a
        //value. Keep this in mind as you explore function return values and expressions next.
        x + 1
    };

    println!("Y value: {:?}", y);
}
