fn main() {
    let message = "Hello, Rust!";
    let shift: u8 = 3;
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

    println!("Original: {}", message);
    println!("Encoded:  {}", result);
}
