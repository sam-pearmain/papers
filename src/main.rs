use clap::{Parser};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    name: String, 
}

fn main() {
    let args = Args::parse();

    println!("hello {}", args.name);
}
