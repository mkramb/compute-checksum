mod collect;

use clap::Parser;
use collect::get_files;

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
    let files = get_files(&args.path);

    println!("Hello {}!", files.unwrap().len());
}
