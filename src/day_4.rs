use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day_4() {
    // File assignments.txt must exist in the current path
    let mut final_count: u32 = 0;
    
    if let Ok(lines) = read_lines("./assignments.txt") {
        for line in lines {
            let fully_contained: bool = get_pair_fully_contained(&line.unwrap());
        }
    }
}

fn get_pair_fully_contained(unformatted_str: &str) -> bool {
    let fully_contained: bool;
    let mut first_assignment = [0,0];
    let mut second_assignment = [0,0];

    for character in unformatted_str.chars() {
        let mut team_switch: bool = false;
        let mut assign_switch: bool = false;

        if character == ',' {
            team_switch = true;
        }
        if character == '-' {
            assign_switch = true;
        }
        if team_switch == false {
            if assign_switch == false {
                first_assignment[0] = character.parse.unwrap();
            }
            else {
                first_assignment[1] = character.parse.unwrap();
            }
        }
        else {
            if assign_switch == false {
                second_assignment[0] = character.parse.unwrap();
            }
            else {
                second_assignment[1] = character.parse.unwrap();
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
