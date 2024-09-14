mod decode;
mod encode;
mod reading;
mod utils;

use std::env;

use clap::{command, ArgGroup, Parser};
use decode::decode;
use encode::encode;
use reading::{read_pgns, save_pgns};

#[derive(Parser, Debug)]
#[command(
    name = "filetochess",
    version = env!("CARGO_PKG_VERSION"),
    group = ArgGroup::new("command")
        .args(&["encode", "decode"])
        .required(true)
        .multiple(false)
)]
struct Cli {
    #[arg(
        short = 'e',
        long = "encode",
        help = "Encodes file to a chess game.",
        value_name = "FILE"
    )]
    encode: Option<String>,

    #[arg(
        short = 'd',
        long = "decode",
        help = "Decodes file from a chess game.",
        value_name = "OUTPUT_FILE"
    )]
    decode: Option<String>,
}

pub fn main() {
    let cli = Cli::parse();

    if cli.encode.is_some() {
        let pgns = encode(&cli.encode.unwrap());

        let _ = save_pgns(&pgns, "./games");

        return;
    }

    if cli.decode.is_some() {
        let out = read_pgns("./games").unwrap();

        decode(&out, &cli.decode.unwrap());
    }
}
