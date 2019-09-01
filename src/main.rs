use morse::code::Code;
use morse::key::*;
use std::str::FromStr;

use std::io::Write;
use std::sync::Arc;
use std::time::Duration;

use tokio::prelude::*;

fn main() {
    let s = "Hello, world!";
    println!("{}", s);

    let p = Code::from_str(&s).unwrap();
    println!("{}", p);

    println!(
        "{}",
        p.timing()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    );

    let k = Arc::new(Key {
        on: || {
            print!("1");
            std::io::stdout().flush().unwrap();
        },
        off: || {
            print!("0");
            std::io::stdout().flush().unwrap();
        },
    });

    let k1 = Arc::clone(&k);
    let k2 = Arc::clone(&k);

    tokio::run(
        transmit_with_dur(k1, p.into_timing(), Duration::from_millis(10))
            .and_then(|_| {
                println!();
                future::ok(())
            })
            .and_then(|_| {
                transmit_with_dur(
                    k2,
                    Code::from_str("Morse code.").unwrap().into_timing(),
                    Duration::from_millis(50),
                )
            })
            .and_then(|_| {
                println!();
                future::ok(())
            }),
    );
}
