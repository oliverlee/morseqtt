use morse::phrase::Phrase;
use std::str::FromStr;

fn main() {
    let s = "Hello, world!";
    println!("{}", s);

    let p = Phrase::from_str(&s).unwrap();
    println!("{}", p);

    println!(
        "{}",
        p.timing()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}
