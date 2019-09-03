use indicatif::{ProgressBar, ProgressStyle};
use morseqtt::code::Code;
use morseqtt::key::*;
use rumqtt::{MqttClient, MqttOptions};
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::prelude::*;

use std::convert::TryInto;

const CLIENT_NAME: &str = "morseqtt";

// concat!() doesn't accept const variables so we define a macro so the values aren't written twice.
macro_rules! default_host {
    () => {
        "localhost"
    };
}
macro_rules! default_port {
    () => {
        1883
    };
}
macro_rules! default_duration_ms {
    () => {
        50
    };
}

fn program_opts() -> getopts::Options {
    let mut opts = getopts::Options::new();

    opts.optopt(
        "h",
        "host",
        concat!("mqtt host to connect to. [", default_host!(), "]"),
        "<host>",
    );
    opts.optopt(
        "p",
        "port",
        concat!("network port to connect to. [", default_port!(), "]"),
        "<port>",
    );
    opts.optopt(
        "d",
        "duration",
        concat!(
            "morse code dot duration, in milliseconds. [",
            default_duration_ms!(),
            "]"
        ),
        "<milliseconds>",
    );
    opts.optflag("", "help", "print this help menu");

    opts
}

fn print_usage(program: &str, opts: &getopts::Options) {
    let brief = format!(
        concat!(
            "Usage: {} [options] <topic> <on_payload> <off_payload>\n\n",
            "Encode input as Morse code and transmit with MQTT."
        ),
        program
    );
    print!("{}", opts.usage(&brief));
}

struct ProgramOptions {
    host: String,
    port: u16,
    duration: Duration,
    topic: String,
    on_payload: String,
    off_payload: String,
}

fn parse_args() -> Option<ProgramOptions> {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let opts = program_opts();
    let mut matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            println!();
            print_usage(&program, &opts);
            return None;
        }
    };
    if matches.opt_present("help") || matches.free.len() != 3 {
        print_usage(&program, &opts);
        return None;
    }

    let host = matches
        .opt_str("host")
        .unwrap_or_else(|| default_host!().to_string());
    let port = matches
        .opt_str("port")
        .map_or_else(|| Ok(default_port!()), |s| s.parse::<u16>());
    let port = match port {
        Ok(p) => p,
        Err(e) => {
            println!("Error parsing 'port': {}", e);
            return None;
        }
    };
    let duration = Duration::from_millis(
        matches
            .opt_str("duration")
            .map(|s| s.parse::<u64>().unwrap())
            .unwrap_or(default_duration_ms!()),
    );
    let off_payload = matches.free.pop().unwrap();
    let on_payload = matches.free.pop().unwrap();
    let topic = matches.free.pop().unwrap();

    Some(ProgramOptions {
        host,
        port,
        duration,
        topic,
        on_payload,
        off_payload,
    })
}

fn stdin_stream() -> tokio::codec::FramedRead<
    std::io::BufReader<tokio::reactor::PollEvented2<tokio_file_unix::File<std::fs::File>>>,
    tokio_file_unix::DelimCodec<tokio_file_unix::Newline>,
> {
    // Convert stdin into a nonblocking file;
    let file = tokio_file_unix::raw_stdin().unwrap();
    let file = tokio_file_unix::File::new_nb(file).unwrap();
    let file = file
        .into_reader(&tokio::reactor::Handle::default())
        .unwrap();

    let line_codec = tokio_file_unix::DelimCodec(tokio_file_unix::Newline);
    tokio::codec::FramedRead::new(file, line_codec)
}

fn progress_bar(message: &str, length: usize) -> ProgressBar {
    // Assume that length is correct for message as we aren't going to convert to a timing phrase again.
    let pb = ProgressBar::new(length.try_into().unwrap());
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{prefix} {wide_bar:.cyan/blue}")
            .progress_chars("##-"),
    );
    pb.set_prefix(&format!("Transmitting: {}", message));

    // We can simply change the style when the transmission is complete.
    pb.set_message(&format!("Transmitted: {}", message));

    pb
}

fn main() {
    let mut args = if let Some(args) = parse_args() {
        args
    } else {
        return;
    };

    let mqtt_options = MqttOptions::new(CLIENT_NAME, &args.host, args.port);
    let client = match MqttClient::start(mqtt_options) {
        Ok((client, _)) => client,
        Err(e) => {
            println!("Error connecting to MQTT broker: {}", e);
            return;
        }
    };
    println!(
        "Connected to {}:{} as {}",
        args.host, args.port, CLIENT_NAME
    );

    // Take topic and payloads from `args`.
    let mut topic: String = "".to_string();
    let mut on_payload: String = "".to_string();
    let mut off_payload: String = "".to_string();
    std::mem::swap(&mut topic, &mut args.topic);
    std::mem::swap(&mut on_payload, &mut args.on_payload);
    std::mem::swap(&mut off_payload, &mut args.off_payload);

    // Create a Key for transmission.
    let k = Arc::new(Mutex::new(MqttKey::new(
        client,
        topic,
        on_payload,
        off_payload,
    )));

    println!("Type something and hit enter to transmit!");
    let task = stdin_stream()
        .for_each(move |line| {
            let s = std::str::from_utf8(&line)
                .unwrap_or_else(|e| {
                    println!("Unable to parse: {}", e);
                    ""
                })
                .trim();

            let code = if let Ok(c) = Code::from_str(&s) {
                c
            } else {
                println!("Input contained invalid characters");
                Code::from_str("").unwrap()
            };

            // Don't spawn this task as we want don't want multiple, simultaneous transmissions.
            let f = if code.is_empty() {
                future::Either::A(future::ok(()))
            } else {
                // Morse code is only uppercase.
                let pb = progress_bar(&s.to_uppercase(), code.timing().count());

                future::Either::B(transmit_with_dur(
                    Arc::clone(&k),
                    code.into_timing(),
                    args.duration,
                    Some(pb),
                ))
            };

            // Convert error type to what FramedRead.for_each expects.
            f.map_err(|_| Error::new(ErrorKind::Other, ""))
        })
        .map_err(|e| panic!("{:?}", e));

    tokio::run(task);
}
