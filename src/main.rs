use morse::code::Code;
use std::str::FromStr;

use tokio::prelude::*;
use tokio::timer::Delay;

use std::time::{Duration, Instant};

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

    let task = stream::iter_ok(p.into_timing())
        .for_each(|x| {
            Delay::new(Instant::now() + Duration::from_millis(100)).and_then(move |_| {
                print!("{}", x.to_string());
                std::io::stdout().flush().unwrap();

                future::ok(())
            })
        })
        .map_err(|_| ());

    println!("starting tokio transmit");
    tokio::run(task);
    println!();
}
