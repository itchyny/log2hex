use clap::{arg, Command};
use log2hex::*;
use std::env;
use thiserror::Error;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), err);
        std::process::exit(1);
    }
}

#[derive(Debug, Error)]
enum Error {
    #[error("invalid place: {0}")]
    Place(#[from] std::num::ParseIntError),
}

fn run() -> Result<(), Error> {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(arg!([PLACE] "Place of digits to calculate (defaults to 0)"))
        .get_matches();
    let place: u32 = match matches.get_one::<String>("PLACE") {
        Some(x) => x.parse()?,
        None => 0,
    };
    print!("{}:", place);
    for i in 0..8 {
        print!(" {}", log2hex(place + 4 * i));
    }
    println!();
    Ok(())
}
