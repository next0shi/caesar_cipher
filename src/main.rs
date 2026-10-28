fn encode(message: &str, shift: i32) -> String {
    let shift = shift.rem_euclid(26) as u8;
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

fn decode(message: &str, shift: i32) -> String {
    encode(message, -shift)
}

fn main() {
    let original = "Hello, Rust!";
    let shift = 3;

    let encoded = encode(original, shift);
    let decoded = decode(&encoded, shift);

    println!("Original: {}", original);
    println!("Encoded:  {}", encoded);
    println!("Decoded:  {}", decoded);
}
