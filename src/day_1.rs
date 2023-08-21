use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day_1(){
    // Store curent highest calories
    let mut high_cal = 0;
    let mut next_number:i32;
    let mut cal_sum:i32 = 0;
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./calories.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(calories) = line {
                    next_number = match calories.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {cal_sum = 0; continue},
                    };
                    cal_sum = next_number + cal_sum;
                    if cal_sum > high_cal{
                        high_cal = cal_sum;
                    }
                }
            }
        }
    println!("Highest calorie is, {}", high_cal);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
