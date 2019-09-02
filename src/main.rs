use morseqtt::code::Code;
use morseqtt::key::*;
use rumqtt::{MqttClient, MqttOptions, QoS};
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::prelude::*;

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

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!(
        concat!(
            "Usage: {} [options] <topic> <on_payload> <off_payload>\n\n",
            "Translate input to morse code and send using MQTT."
        ),
        program
    );
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = program_opts();
    let mut matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            println!();
            print_usage(&program, opts);
            return;
        }
    };
    if matches.opt_present("help") {
        print_usage(&program, opts);
        return;
    }
    if matches.free.len() != 3 {
        print_usage(&program, opts);
        return;
    }

    let host = matches
        .opt_str("host")
        .unwrap_or(default_host!().to_string());
    let port = matches
        .opt_str("port")
        .map(|s| s.parse::<u16>().unwrap())
        .unwrap_or(default_port!());
    let duration = Duration::from_millis(
        matches
            .opt_str("duration")
            .map(|s| s.parse::<u64>().unwrap())
            .unwrap_or(default_duration_ms!()),
    );
    let off_payload = matches.free[0].as_str();
    let on_payload = matches.free[1].as_str();
    let topic = matches.free[2].as_str();
    println!("topic: {}", topic);
    println!("on payload: {}", on_payload);
    println!("off payload: {}", off_payload);
    println!("duration: {:?}", duration);
    println!("host: {}", host);
    println!("port: {}", port);

    let mqtt_options = MqttOptions::new(CLIENT_NAME, &host, port);
    let (mut client1, _) = MqttClient::start(mqtt_options).unwrap();
    let mut client2 = client1.clone();
    println!("Connected to {}:{} as {}", host, port, CLIENT_NAME);

    // Create a Key for transmission.
    let k = Arc::new(Mutex::new(Key {
        on: move || {
            let payload = "ON";
            client1
                .publish("hello/world", QoS::AtLeastOnce, false, payload)
                .unwrap();
        },
        off: move || {
            let payload = "OFF";
            client2
                .publish("hello/world", QoS::AtLeastOnce, false, payload)
                .unwrap();
        },
    }));

    // Convert stdin into a nonblocking file;
    let file = tokio_file_unix::raw_stdin().unwrap();
    let file = tokio_file_unix::File::new_nb(file).unwrap();
    let file = file
        .into_reader(&tokio::reactor::Handle::default())
        .unwrap();

    println!("Type something and hit enter to transmit!");
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
                duration,
            )
            .and_then(|_| {
                println!("Transmission complete.");
                future::ok(())
            }) // Convert error type to what FramedRead.for_each expects.
            .map_err(|_| Error::new(ErrorKind::Other, ""))
        })
        .map_err(|e| panic!("{:?}", e));

    tokio::run(task);
}
