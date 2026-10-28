use std::io;

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

fn ask(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    println!("=== Caesar Cipher ===");

    let mode = ask("Type 'e' to encode or 'd' to decode:");
    let message = ask("Enter your message:");
    let shift_text = ask("Enter the shift number:");

    let shift: i32 = match shift_text.parse() {
        Ok(number) => number,
        Err(_) => {
            println!("That's not a valid number.");
            return;
        }
    };

    if mode == "e" {
        println!("Result: {}", encode(&message, shift));
    } else if mode == "d" {
        println!("Result: {}", decode(&message, shift));
    } else {
        println!("Please type 'e' or 'd'.");
    }
}
