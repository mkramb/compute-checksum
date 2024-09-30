use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, required = true)]
    path: Vec<String>,

    #[clap(short, long)]
    exclude: Vec<String>
}

fn main() {
    let args = Args::parse();

    println!("Hello {}!", args.exclude.concat());
}