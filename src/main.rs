use std::result;

use clap::Parser;

//cargo r -- some-pattern some-file
#[derive(Parser)]
struct Cli {
    year: i32,
}

fn main() {
    let args = Cli::parse();

    if args.year % 4 == 0 && args.year % 100 != 0 || args.year % 400 == 0 {
        println!("The year {} is a leap year", args.year);
    } else {
        println!("The year {} is not a leap year", args.year);
    }
}
