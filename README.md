# Caesar Cipher in Rust

A small command-line tool that encodes and decodes messages using the Caesar cipher, built as a beginner project to learn Rust.

## What is a Caesar cipher?

A Caesar cipher hides a message by shifting every letter a fixed number of places along the alphabet. The number of places is the **key**.

With a shift of 3:

```
A -> D    B -> E    C -> F    ...    X -> A    Y -> B    Z -> C
```

So `Hello, Rust!` becomes `Khoor, Uxvw!`. To decode, shift the same number of places backwards.

> **Note:** This is a learning project, not real security. A Caesar cipher only has 25 possible keys and is trivial to break. Never use it to protect anything private.

## Features

- Encode a message with any shift
- Decode a message with the same shift
- Works with uppercase and lowercase letters
- Leaves spaces, numbers, and punctuation unchanged
- Handles large and negative shifts (for example `29` behaves like `3`, and `-3` shifts backwards)
- Interactive menu that loops until you quit
- Friendly handling of invalid input instead of crashing

## Requirements

- [Rust](https://rustup.rs) (this installs both `rustc` and `cargo`)

Check your install:

```bash
rustc --version
cargo --version
```

## Run it

```bash
git clone <your-repo-url>
cd caesar_cipher
cargo run
```

### Example session

```
=== Caesar Cipher ===

Type 'e' to encode, 'd' to decode, or 'q' to quit:
e
Enter your message:
Hello, Rust!
Enter the shift number:
3
Result: Khoor, Uxvw!

Type 'e' to encode, 'd' to decode, or 'q' to quit:
d
Enter your message:
Khoor, Uxvw!
Enter the shift number:
3
Result: Hello, Rust!

Type 'e' to encode, 'd' to decode, or 'q' to quit:
q
Goodbye!
```

## Project structure

```
caesar_cipher/
├── Cargo.toml    # project settings
└── src/
    └── main.rs   # all the code
```

## How it works

### 1. Letters are numbers

Computers store characters as numbers (ASCII). `A` to `Z` are 65 to 90, and `a` to `z` are 97 to 122. That means shifting a letter is just arithmetic.

### 2. The shifting recipe

For each letter, `encode` does four things:

1. Find its position in the alphabet (`A` = 0 ... `Z` = 25) by subtracting `b'A'` (or `b'a'` for lowercase).
2. Add the shift.
3. Wrap around past `Z` using `% 26`.
4. Convert back to a letter by adding `b'A'` (or `b'a'`) again.

Example: `Z` is position 25, plus a shift of 3 is 28, and `28 % 26 = 2`, which is `C`.

### 3. Handling any shift

The shift is stored as an `i32` so it can be negative. Before use, `shift.rem_euclid(26)` turns any number into an equivalent shift between 0 and 25.

### 4. Decoding is shifting backwards

`decode` reuses `encode` with the opposite shift (`26 - shift.rem_euclid(26)`), so the shifting logic is only written once.

### 5. The main loop

`main` repeatedly asks for a mode (`e`, `d`, or `q`), a message, and a shift. It reads keyboard input with a small helper function, `ask`, and uses `match` to handle numbers that can't be parsed.

## Functions

| Function | What it does |
|---|---|
| `encode(message, shift)` | Shifts every letter forward and returns a new `String` |
| `decode(message, shift)` | Reverses `encode` by shifting backwards |
| `ask(prompt)` | Prints a question, reads one line of input, and returns it trimmed |
| `main()` | Runs the interactive menu loop |

## What I learned

**Rust basics**
- Installing Rust and creating a project with `cargo new`
- Building and running with `cargo run`
- Variables with `let`, and why they are immutable by default (`mut` makes them changeable)
- Shadowing: reusing a variable name with a new value or type

**Types**
- `&str` and `String` for text
- `char` for a single character
- `i32` for signed whole numbers and `u8` for small unsigned ones
- Converting between types with `as`
- `b'A'` as the ASCII value of a letter

**Control flow**
- `for` loops with `.chars()` to walk through text
- `loop`, `break`, and `continue`
- `if` / `else if` / `else`, including `if` as an expression that returns a value
- `&&` (and) and `||` (or)
- `match` and `Result` (`Ok` / `Err`) for handling things that might fail

**Functions and ownership**
- Writing functions with parameters and return types
- Returning a value from the last line (no semicolon)
- Borrowing with `&` (read only) and `&mut` (read and change)

**Working with text and input**
- Building a new `String` with `push`
- Reading keyboard input with `std::io`
- Cleaning input with `trim` and converting it with `parse`
- Using math with `%` and `rem_euclid` to wrap around the alphabet

## Possible improvements

- Add a crack mode
- Read and write messages from files
- Support numbers or a custom alphabet
- Write tests with `cargo test`

## License

Free to use and learn from.
