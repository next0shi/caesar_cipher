fn encode(message: &str, shift: u8) -> String {
    let mut result = String::new();

    for c in message.chars() {
        if c.is_ascii_uppercase() {
            let position = c as u8 - b'A';
            let new_position = (position + shift) % 26;
            result.push((new_position + b'A') as char);
        } else if c.is_ascii_lowercase() {
            let position = c as u8 - b'a';
            let new_position = (position + shift) % 26;
            result.push((new_position + b'a') as char);
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    let message = "Hello, Rust!";
    let encoded = encode(message, 3);

    println!("Original: {}", message);
    println!("Encoded:  {}", encoded);
}
