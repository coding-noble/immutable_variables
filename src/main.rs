#![deny(clippy::all)]

fn main() {
    // In Rust when declaring a variable by default it is immutable.
    // Immutable variables are the same thing as constant or permanent variables that can't be changed.
    // Using the keyword let we can make a variable named 'x' with the value 5.
    let x = 5;

    // Attempting to change the value of 'x' will result in a compilation error.
    // x = 10; (error)

    // Printing the value of the immutable variable 'x'.
    println!("The value of x is: {}", x);

    // unlike other C based languages we don't have to declare what type of variable we are making.
    let name = "myName";

    println!("{}", name);

    // but you still can specify what type of value it will hold with the following syntax.
    let only_integers: i32 = 10;

    println!("{}", only_integers);
}
