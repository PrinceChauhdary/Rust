fn main() {
    // Example 1: Ownership transfer
    let s1 = String::from("hello");
    let s2 = s1; // Ownership of s1 is moved to s2
    // println!("{}", s1); // Error: s1 is no longer valid
    println!("{}", s2);

    // Example 2: Cloning
    let s3 = String::from("world");
    let s4 = s3.clone(); // Deep copy
    println!("s3 = {}, s4 = {}", s3, s4);

    // Example 3: Borrowing with references
    let s5 = String::from("borrow");
    let len = calculate_length(&s5); // Borrowing s5
    println!("The length of '{}' is {}", s5, len);

    // Example 4: Mutable borrowing
    let mut s6 = String::from("mutable");
    change(&mut s6); // Mutable borrow
    println!("Changed string: {}", s6);

    // Example 5: Multiple immutable references
    let s7 = String::from("immutable");
    let r1 = &s7;
    let r2 = &s7;
    println!("r1 = {}, r2 = {}", r1, r2);

    // Example 6: Mutable and immutable references conflict
    let mut s8 = String::from("conflict");
    let r3 = &s8;
    // let r4 = &mut s8; // Error: Cannot borrow as mutable while immutable borrow exists
    println!("{}", r3);

    // Example 7: Returning ownership
    let s9 = gives_ownership();
    println!("Returned ownership: {}", s9);

    // Example 8: Passing ownership to a function
    let s10 = String::from("ownership");
    takes_ownership(s10);
    // println!("{}", s10); // Error: s10 is no longer valid

    // Example 9: Slices and borrowing
    let s11 = String::from("hello world");
    let word = first_word(&s11);
    println!("First word: {}", word);

    // Example 10: Struct and borrowing
    let rect = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", area(&rect));
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" changed");
}

fn gives_ownership() -> String {
    let s = String::from("ownership given");
    s
}

fn takes_ownership(s: String) {
    println!("Took ownership of: {}", s);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}