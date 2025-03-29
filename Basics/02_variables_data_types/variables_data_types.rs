fn main() {
    // Immutable variable
    let x = 5;
    println!("The value of x is: {}", x);

    // Mutable variable
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 20;  // Allowed because y is mutable
    println!("The new value of y is: {}", y);

    // Constants
    const PI: f64 = 3.14159;
    println!("The value of PI is: {}", PI);

    // Data types
    let is_rust_fun: bool = true;  // Boolean
    let letter: char = 'R';  // Character
    let num: i32 = 42;  // Integer
    let decimal: f64 = 3.14;  // Floating point number

    println!("Boolean: {}, Character: {}, Integer: {}, Float: {}", is_rust_fun, letter, num, decimal);
}
