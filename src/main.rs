use std::{io::{BufReader}, };
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

use structopt::StructOpt;

// Our CLI arguments. (help and version are automatically generated)
// Documentation on how to use:
// https://docs.rs/structopt/0.2.10/structopt/index.html#how-to-derivestructopt
#[derive(StructOpt, Debug)]
#[structopt(name = "dir-scanner", about = "Directory Scanner")]
struct Cli {
    #[structopt(short="l", long="list")]
    list: String,
    #[structopt(short="u", long="url")]
    url: String,
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let args = Cli::from_args();
    let wordlist: Vec<String> = lines_from_file(args.list);
    let url = args.url;
}

