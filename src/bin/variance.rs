use clap::Parser;
use variation::opts::Opts;

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}
