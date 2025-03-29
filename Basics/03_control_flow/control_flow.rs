// control_flow.rs

fn main() {
    // Example 1: If/Else Condition
    let health = 45;
    if health > 50 {
        println!("The warrior attacks bravely! ⚔️");
    } else if health >= 20 {
        println!("The warrior attacks cautiously. 🛡");
    } else {
        println!("The warrior retreats to heal! 🏥");
    }

    // Example 2: Match Expression
    let spell = "fire";
    match spell {
        "fire" => println!("🔥 Fireball!"),
        "ice" => println!("❄️ Ice Blast!"),
        "lightning" => println!("⚡ Thunder Strike!"),
        _ => println!("🔮 Unknown spell..."),
    }

    // Example 3: While Loop
    let mut monster_health = 30;
    while monster_health > 0 {
        println!("⚔️ The warrior attacks! Monster’s health: {}", monster_health);
        monster_health -= 5;
    }
    println!("🎉 The monster is defeated!");

    // Example 4: For Loop
    for potion in 1..=5 {
        println!("🧪 Selling potion {}", potion);
    }
}
// Example 5: Loop with Break and Continue
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("Skipping count 3!");
            continue; // Skip the rest of the loop for this iteration
        }
        if count > 5 {
            break; // Exit the loop when count exceeds 5
        }
        println!("Count: {}", count);
    }
    println!("Loop ended at count: {}", count);
}   
// Example 6: Using if let for Pattern Matching
    let some_value = Some(10);
    if let Some(x) = some_value {
        println!("The value is: {}", x);
    } else {
        println!("No value found.");
    }

    // Example 7: Using match with Destructuring
    let point = (3, 5);
    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On the X-axis at {}", x),
        (0, y) => println!("On the Y-axis at {}", y),
        (x, y) => println!("Point at ({}, {})", x, y),
    }
}
// Example 8: Using match with Guards
    let number = 7;
    match number {
        n if n < 0 => println!("Negative number"),
        n if n == 0 => println!("Zero"),
        n if n % 2 == 0 => println!("Even number"),
        _ => println!("Odd number"),
    }
}
// Example 9: Using match with Ranges
    let age = 25;
    match age {
        0..=12 => println!("Child"),
        13..=19 => println!("Teenager"),
        20..=64 => println!("Adult"),
        _ => println!("Senior"),
    }
}  
// Example 10: Using match with Option
    let maybe_number = Some(42);
    match maybe_number {
        Some(n) if n > 0 => println!("Positive number: {}", n),
        Some(n) => println!("Number: {}", n),
        None => println!("No number provided"),
    }
}
// Example 11: Using match with Result
    let result: Result<i32, &str> = Ok(100);
    match result {
        Ok(value) => println!("Success! Value: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}   
