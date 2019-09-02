use morse::code::Code;
use morse::key::*;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::prelude::*;

const DOT_DURATION: Duration = Duration::from_millis(50);

fn main() {
    // Create a Key for transmission.
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

    // Convert stdin into a nonblocking file;
    let file = tokio_file_unix::raw_stdin().unwrap();
    let file = tokio_file_unix::File::new_nb(file).unwrap();
    let file = file
        .into_reader(&tokio::reactor::Handle::default())
        .unwrap();

    println!("Type something and hit enter!");
    let line_codec = tokio_file_unix::DelimCodec(tokio_file_unix::Newline);
    let framed_read = tokio::codec::FramedRead::new(file, line_codec);

    let task = framed_read
        .for_each(move |line| {
            let s = std::str::from_utf8(&line).unwrap().trim();
            println!("Transmitting: {}", s);

            // Don't spawn this task as we want don't want multiple, simultaneous transmissions.
            transmit_with_dur(
                Arc::clone(&k),
                Code::from_str(s).unwrap().into_timing(),
                DOT_DURATION,
            )
            .and_then(|_| {
                println!();
                future::ok(())
            }) // Convert error type to what FramedRead.for_each expects.
            .map_err(|_| Error::new(ErrorKind::Other, ""))
        })
        .map_err(|e| panic!("{:?}", e));

    tokio::run(task);
}
