# 🦀 Rust Ownership & Borrowing

## 📖 Introduction
Ownership is Rust’s unique memory management system that ensures memory safety without a garbage collector.
It has three main rules:

1. Each value in Rust has a single **owner**.
2. When the owner goes out of scope, Rust automatically **frees the memory**.
3. **Ownership can be transferred (moved) or borrowed**.

Understanding **ownership** and **borrowing** is crucial to writing efficient and safe Rust programs.

---

## 🎯 Key Concepts

### 1️⃣ Ownership Basics
```rust
fn main() {
    let name = String::from("Rust"); // `name` owns the string
    println!("Hello, {}!", name);
} // `name` goes out of scope, and memory is freed
```

### 2️⃣ Moving Ownership
```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // Ownership moves from s1 to s2
    
    // println!("{}", s1); // ❌ Error! `s1` is no longer valid
    println!("{}", s2); // ✅ Works fine
}
```

### 3️⃣ Cloning (Deep Copy)
```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // Creates a deep copy
    println!("s1: {}, s2: {}", s1, s2); // Both are valid
}
```

### 4️⃣ Borrowing with References
```rust
fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn main() {
    let s = String::from("Borrowed");
    print_length(&s); // Pass reference, ownership is not moved
    println!("Still valid: {}", s); // ✅ s is still valid
}
```

### 5️⃣ Mutable Borrowing
```rust
fn change(s: &mut String) {
    s.push_str(" is awesome!");
}

fn main() {
    let mut s = String::from("Rust");
    change(&mut s); // Borrow mutably
    println!("{}", s); // ✅ Modified value
}
```

---

🚀 **Master ownership & borrowing, and become a true Rustorian!**

