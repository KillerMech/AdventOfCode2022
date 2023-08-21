use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day_1_2(){
    // Store curent highest calories
    let mut next_number:i32;
    let mut cal_sum:i32 = 0;
    let mut cal_ranks = [0,0,0];
    let mut total_cals = 0;
    // File calories.txt must exist in the current path
    if let Ok(lines) = read_lines("./calories.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(calories) = line {
                    next_number = match calories.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {cal_sum = 0; continue},
                    };
                    cal_sum = next_number + cal_sum;
                    for index in 0..cal_ranks.len(){
                        if cal_sum > cal_ranks[index]{
                            cal_ranks[index] = cal_sum;
                            break;
                        }
                    }
                }
            }
        }
    println!("Printing top three calories holders!");
    for index in 0..cal_ranks.len(){
        println!("Cal {} was: {}",index ,cal_ranks[index]);
        total_cals = total_cals + cal_ranks[index];
    }
    println!("That makes the total calories of the top three elves: {}", total_cals);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
