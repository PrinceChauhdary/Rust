# 📘 Rust Functions: A Complete Guide  

Functions are the building blocks of any Rust program. They help in organizing code, reducing repetition, and improving readability. In Rust, functions are defined using the `fn` keyword.  

---

## 🛠 Function Syntax  

```rust
fn function_name(parameter1: Type, parameter2: Type) -> ReturnType {
    // Function body
    return value; // Optional: The last expression is implicitly returned
}
```

- `fn` → Declares a function.  
- `function_name` → The name of the function.  
- `parameter1, parameter2` → Inputs with their types.  
- `-> ReturnType` → Specifies the return type (if any).  
- `{}` → Function body containing logic.  

---

## 🔹 Types of Functions in Rust  

### 1️⃣ **Basic Function**  
A simple function with no parameters and no return value.  

```rust
fn greet() {
    println!("Hello, Rustacean! 🦀");
}

fn main() {
    greet(); // Calling the function
}
```

📝 **Explanation:**  
- The `greet()` function prints a message.  
- It does not take any parameters or return a value.  

---

### 2️⃣ **Function with Parameters**  
Functions can take input parameters.  

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("Alice");
}
```

📝 **Explanation:**  
- `name: &str` → A string slice parameter is passed.  
- `"Alice"` is provided as an argument when calling `greet()`.  

---

### 3️⃣ **Function with Return Value**  
Functions can return values using the `->` notation.  

```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    let sum = add(5, 7);
    println!("Sum is: {}", sum);
}
```

📝 **Explanation:**  
- `add()` takes two integers and returns their sum.  
- `-> i32` specifies the return type.  
- The `return` keyword is optional (the last expression is implicitly returned).  

---

### 4️⃣ **Implicit Return (Without `return`)**  
Rust allows returning values without using the `return` keyword.  

```rust
fn multiply(a: i32, b: i32) -> i32 {
    a * b // No semicolon = implicit return
}

fn main() {
    let result = multiply(3, 4);
    println!("Result: {}", result);
}
```

📝 **Explanation:**  
- The last expression (`a * b`) is returned automatically.  
- If a semicolon was added (`a * b;`), Rust would throw an error.  

---

### 5️⃣ **Functions with Multiple Return Values (Tuples)**  
Rust allows returning multiple values using tuples.  

```rust
fn get_coordinates() -> (i32, i32) {
    (10, 20)
}

fn main() {
    let (x, y) = get_coordinates();
    println!("Coordinates: ({}, {})", x, y);
}
```

📝 **Explanation:**  
- The function returns a tuple `(10, 20)`.  
- Destructuring is used to extract `x` and `y`.  

---

### 6️⃣ **Recursive Functions**  
A function that calls itself is called **recursive**.  

```rust
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    println!("Factorial of 5 is: {}", factorial(5));
}
```

📝 **Explanation:**  
- If `n == 0`, return `1`.  
- Otherwise, call `factorial(n-1)` recursively.  

---

### 7️⃣ **Functions with Closures**  
Rust supports anonymous functions (closures).  

```rust
fn main() {
    let square = |x: i32| -> i32 { x * x };
    println!("Square of 4: {}", square(4));
}
```

📝 **Explanation:**  
- `|x: i32| -> i32 { x * x }` is a **closure** (anonymous function).  
- It takes `x` and returns `x * x`.  

---

## 🚀 Conclusion  
- Functions help **organize** code efficiently.  
- Rust supports **parameters, return values, recursion, and closures**.  
- Using functions improves **code readability** and **modularity**.  

✅ **Now it's your turn!** Try creating your own functions and experiment with different return types and logic.  

