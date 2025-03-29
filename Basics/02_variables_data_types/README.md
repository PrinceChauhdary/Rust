# 🦀 Lesson 2: Variables & Data Types in Rust
## 📖 Introduction
Rust is a statically typed language, meaning that variables must have a specific data type. However, Rust can infer types automatically in many cases.

## 📝 Code Example
```rust
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

```
##  Run the program:
rustc variables_data_types.rs <br>
.\variables_data_types.exe <br>

---

✔️ let x = 5; → Immutable variable (cannot be changed).  
✔️ let mut y = 10; → Mutable variable (can be changed later).  
✔️ const PI: f64 = 3.14159; → Constant (requires explicit type).  
✔️ Rust has different data types, including:  

Integer Types: i32, u64, etc.

Floating-Point Types: f32, f64.

Boolean Type: bool.

Character Type: char.

---