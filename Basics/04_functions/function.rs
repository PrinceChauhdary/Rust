// Example 1: Basic Function
fn greet() {
    println!("Hello, world!");
}

// Example 2: Function with Parameters
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Example 3: Function with Return Value
fn square(x: i32) -> i32 {
    x * x
}

// Example 4: Function with Multiple Return Values
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

// Example 5: Recursive Function
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Example 6: Function with String Parameter
fn greet_user(name: &str) {
    println!("Hello, {}!", name);
}

// Example 7: Function with Default Behavior
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// Example 8: Function with a Closure
fn apply<F>(x: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

// Example 9: Function with Mutable Reference
fn increment(value: &mut i32) {
    *value += 1;
}

// Example 10: Generic Function
fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    greet();
    println!("Sum: {}", add(5, 3));
    println!("Square: {}", square(4));
    let (min, max) = min_max(10, 20);
    println!("Min: {}, Max: {}", min, max);
    println!("Factorial: {}", factorial(5));
    greet_user("Alice");
    println!("Is 4 even? {}", is_even(4));
    println!("Apply closure: {}", apply(5, |x| x * 2));
    let mut value = 10;
    increment(&mut value);
    println!("Incremented value: {}", value);
    println!("Largest: {}", largest(10, 20));
}