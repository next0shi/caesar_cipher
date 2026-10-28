fn main() {
    let letter = 'H';
    let shift: u8 = 3;

    let number = letter as u8; // 72
    let position = number - b'A'; // 72 - 65 = 7
    let new_position = (position + shift) % 26; // (7 + 3) % 26 = 10
    let new_number = new_position + b'A'; // 10 + 65 = 75
    let new_letter = new_number as char; // 'K'

    println!("{} shifted by {} becomes {}", letter, shift, new_letter);
}
