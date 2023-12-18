mod search;

use std::{env, fs::File, io::BufReader};
use search::search_stream;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: {} [needle] [haystack]", args[0]);
        return;
    }

    let needle = &args[1];
    let haystack = &args[2];
    match File::open(&haystack) {
        Ok(f) => {
            let mut reader = BufReader::new(f);
            let match_offsets = search_stream(&needle, &mut reader);
            println!("offsets: {:?}", match_offsets);
        },
        Err(err) => eprintln!("error opening file '{}': {}", haystack, err),
    };
}
