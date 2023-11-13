mod strtool;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("no string provided via command line arguments.");
        return;
    }

    println!("{}", crate::strtool::alternating_chars(&args[1]));
}
