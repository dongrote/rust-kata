mod sort;

use std::env;

fn main() {
    let nums = collect_numeric_args(&env::args().collect::<Vec<String>>()[1..]);

    println!("{:?}", crate::sort::insertion_sort(&nums));
}

fn collect_numeric_args(args: &[String]) -> Vec<i16> {
    let mut numeric_args = Vec::new();
    for arg in args {
        match arg.parse() {
            Ok(n) =>  numeric_args.push(n),
            Err(_) => {},
        }
    }

    numeric_args
}
