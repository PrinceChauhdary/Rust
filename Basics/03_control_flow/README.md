# 🦀 Lesson 3: Control Flow in Rust (if/else, loops)

## 📖 Introduction
Control flow in Rust allows you to make decisions and execute code conditionally. This lesson covers:
- `if` and `else` statements
- `match` expressions
- Loops: `while`, `for`, `loop`
- Pattern matching techniques

---

## 🚀 Topics Covered

### 1️⃣ If/Else Conditionals
The `if` statement helps execute code based on conditions.

```rust
let health = 45;

if health > 50 {
    println!("The warrior attacks bravely! ⚔️");
} else if health >= 20 {
    println!("The warrior attacks cautiously. 🛡");
} else {
    println!("The warrior retreats to heal! 🏥");
}
```
# 2️⃣ Match Expressions
## match is a powerful alternative to if/else in Rust.
```rust
let spell = "fire";

match spell {
    "fire" => println!("🔥 Fireball!"),
    "ice" => println!("❄️ Ice Blast!"),
    "lightning" => println!("⚡ Thunder Strike!"),
    _ => println!("🔮 Unknown spell..."),
}
```

# 3️⃣ While Loop
## Executes code repeatedly while a condition is true.

```rust
let mut monster_health = 30;

while monster_health > 0 {
    println!("⚔️ The warrior attacks! Monster’s health: {}", monster_health);
    monster_health -= 5;
}

println!("🎉 The monster is defeated!");
```
# 4️⃣ For Loop
## Iterates over a range or collection.
``` rust
for potion in 1..=5 {
    println!("🧪 Selling potion {}", potion);
}
```
# 5️⃣ Loop with break and continue
## Use break to exit a loop and continue to skip an iteration.
``` rust
let mut count = 0;

loop {
    count += 1;
    if count == 3 {
        println!("Skipping count 3!");
        continue;
    }
    if count > 5 {
        break;
    }
    println!("Count: {}", count);
}

println!("Loop ended at count: {}", count);

```
# 6️⃣ Pattern Matching with if let
## Simplified pattern matching for Option types.

```rust 
let some_value = Some(10);

if let Some(x) = some_value {
    println!("The value is: {}", x);
} else {
    println!("No value found.");
}
```
# 7️⃣ Pattern Matching with match
## Matches tuples and other structures.

```rust
let point = (3, 5);

match point {
    (0, 0) => println!("Origin"),
    (x, 0) => println!("On the X-axis at {}", x),
    (0, y) => println!("On the Y-axis at {}", y),
    (x, y) => println!("Point at ({}, {})", x, y),
}
```
