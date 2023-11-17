use std::path::Path;

use timetrack::{
    name,
    types::timetrackerstores::TimeTrackerFileStore};

fn main() {
    let store = TimeTrackerFileStore::from_file(Path::new("foo.txt")).expect("Error opening foo.txt");
    println!("{}: cli", name());
    println!("{:?}", store.duration().to_string());
}
