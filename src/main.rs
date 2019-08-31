use morse::phrase::Phrase;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");

    println!("{}", Phrase::from_str("Hello, world!").unwrap());
}
