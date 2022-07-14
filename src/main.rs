use std::{io::BufReader};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use error_chain::error_chain;
use structopt::StructOpt;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "dir-scanner", about = "Directory Scanner")]
struct Cli {
    #[structopt(short="l", long="list")]
    list: String,
    #[structopt(short="u", long="url")]
    url: String,
    #[structopt(short="s", long="status", help="Only print specific status code")]
    status: Option<u16>,
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn send_request(url: &str) -> Result<u16> {
    let response = reqwest::blocking::get(url)?;
    Ok(response.status().as_u16())
}

fn main() {
    let args = Cli::from_args();
    let wordlist: Vec<String> = lines_from_file(args.list);
    let url: &str = args.url.as_str();
    let status = match args.status {
        Some(s) => s,
        None => 0,
    };

    for dir in wordlist {
        let url_with_dir = format!("{}/{}", url, dir);
        let status_code = send_request(url_with_dir.as_str()).expect("Failed");

        if status != 0 {
            if status == status_code {
                println!("{} /{}", status_code, dir);
            }
        } else {
            println!("{} /{}", status_code, dir);
        }
    }
}

