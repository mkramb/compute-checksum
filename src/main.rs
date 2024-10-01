mod files;
mod hash;

use clap::Parser;
use files::collect_files;
use hash::hash_files;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, required = true)]
    path: String,

    #[clap(short, long)]
    exclude: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let files = collect_files(&args.path);
    let hashes = hash_files(files).await;

    for result in hashes {
        match result {
            Ok(line) => println!("{}", line),
            Err(_e) => return (),
        }
    }

    println!("Done");
}
