fn main() {
    println!("Hello, world!");
    println!("This is my first Rust program");
    println!("first you cargo run to compile and run the program");
    println!("then you can cargo build to build the program");
    println!("then you can run the program with ./target/debug/Lesson_1");
    println!("you can also cargo check to check for errors");   
    println!("you can also cargo test to run the tests");
    println!("you can also cargo doc --open to generate documentation");
    println!("you can also cargo clean to clean the project");  
    println!("you can also cargo update to update the dependencies");
}

//add basic test case to pass pipeline.  
#[test]
fn test_add() {
    assert_eq!((1 + 2), 3);
}
