fn main() {
    println!("Hello, world!");
    println!("This is my first Rust program");
    println!("first you cargo run to compile and run the program");
}

//add basic test case to pass pipeline.  
#[test]
fn test_add() {
    assert_eq!((1 + 2), 3);
}
