use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day_4() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./assignments.txt") {
        let first_team: u32;2;
        let second_team: u32;2;
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
