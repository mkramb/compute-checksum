mod file;
mod hash;

use std::io::{BufReader, Cursor};

use clap::Parser;
use file::collect_files;
use hash::{hash_files,compute_hash};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, required = true)]
    path: String,

    #[clap(short, long)]
    exclude: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let files = collect_files(&args.path);
    let hashes = hash_files(files).join("");

    let cursor = Cursor::new(hashes.as_bytes());
    let reader = BufReader::new(cursor);

    match compute_hash(reader) {
        Ok(hash) => println!("{}", hash),
        Err(e) => eprintln!("Problem parsing file: {e}")
    }
}
