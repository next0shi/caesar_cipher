fn main() {
    let message = "Hello, Rust!";
    let mut result = String::new();

    for c in message.chars() {
        result.push(c);
    }

    println!("Original: {}", message);
    println!("Copy:     {}", result);
}
