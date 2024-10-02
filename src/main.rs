mod file;
mod hash;

use clap::Parser;
use file::collect_files;
use std::io::{BufReader, Cursor};
use hash::{compute_hash, hash_files};

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

    let file_hashes = hash_files(collect_files(&args.path, &args.exclude)).join("");
    let final_hash = compute_hash(BufReader::new(Cursor::new(file_hashes.as_bytes())));

    match final_hash {
        Ok(hash) => println!("{}", hash),
        Err(e) => eprintln!("Problem calculating hash: {e}"),
    }
}
