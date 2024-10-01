mod file;
mod hash;

use clap::Parser;
use file::collect_files;
use hash::hash_files;

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
    let hashes = hash_files(files);

    for result in hashes {
        println!("{}", result);
    }

    println!("Done");
}
