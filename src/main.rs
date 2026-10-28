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
    encode(message, 26 - shift.rem_euclid(26))
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

    loop {
        println!();
        let mode = ask("Type 'e' to encode, 'd' to decode, or 'q' to quit:").to_lowercase();

        if mode == "q" {
            println!("Goodbye!");
            break;
        }

        if mode != "e" && mode != "d" {
            println!("Please type 'e', 'd' or 'q'.");
            continue;
        }

        let message = ask("Enter your message:");
        let shift_text = ask("Enter the shift number:");

        let shift: i32 = match shift_text.parse() {
            Ok(number) => number,
            Err(_) => {
                println!("That's not a valid number.");
                continue;
            }
        };

        let result = if mode == "e" {
            encode(&message, shift)
        } else {
            decode(&message, shift)
        };

        println!("Result: {}", result);
    }
}
