use morse::word::Word;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");

    let w = "MORSE";
    let m = Word::from_str(&w).unwrap();
    println!("Morse code for '{}':{}", w, m);
}
