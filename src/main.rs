mod decode;
mod encode;
mod reading;
mod utils;

use std::{env, path::Path};
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
        value_names = &["FILE", "OUTPUT_FOLDER"]
    )]
    encode: Option<Vec<String>>,

    #[arg(
        short = 'd',
        long = "decode",
        help = "Decodes file from a chess game.",
        value_names = &["FOLDER", "OUTPUT_FILE"]
    )]
    decode: Option<Vec<String>>,
}

pub fn main() {
    let cli = Cli::parse();

    if let Some(encode_args) = cli.encode {
        let input_file = encode_args.get(0).unwrap();
        let folder_path_user = encode_args.get(1).unwrap();

        let folder_path = Path::new(folder_path_user);

        let pgns = encode(input_file);

        let _ = save_pgns(&pgns, folder_path);

        return;
    }

    if let Some(decode_args) = cli.decode.as_ref() {
        let output_file = decode_args.get(1).unwrap();
        let folder_path_user = decode_args.get(0).unwrap();
        let folder_path = Path::new(folder_path_user);

        let out = read_pgns(folder_path).unwrap();

        decode(&out, output_file);
    }
}
