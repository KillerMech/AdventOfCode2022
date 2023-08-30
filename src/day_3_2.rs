use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day_3_2(){
    let mut elf_counter = 0;
    //We know exactly what type of data we are ingesting and how much
    //so an array is very appropriate here
    let mut elf_rucksacks: [String; 3] = ["_".to_string(),"_".to_string(),"_".to_string()];
    let mut rucksack_sum = 0;
    // File rucksacks.txt must exist in the current path
    if let Ok(lines) = read_lines("./rucksacks.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(current_sack) = line {
                //convert the line to a string so we can work with it
                let current_string = current_sack.to_string();
                //put the string in each part of our array
                elf_rucksacks[elf_counter] = current_string;
                elf_counter += 1;
                //we know we've consumed 3 sacks, so we'll evaluate this set
                if elf_counter == 3 {
                    elf_counter = 0;
                    //Send our array to the function that will return the badge
                    let rucksack_return = rucksack_badge(elf_rucksacks.clone());
                    //Same as last time for our calculations
                    if rucksack_return.is_uppercase() {
                        rucksack_sum += rucksack_return as u32 - 38;
                    } else {
                        rucksack_sum += rucksack_return as u32 - 96;
                    }
                }
            }
        }
    }
    println!("Total sum of rucksacks is {}", rucksack_sum);
}

//Function that returns the badge we use to calculate score
fn rucksack_badge(rucksack: [String; 3]) -> char {
    let mut badge: char = '_';

    //use chars() and contains() functions with some logic to find the badge
    for ch in rucksack[0].chars() {
        if rucksack[1].contains(ch) && rucksack[2].contains(ch) {
            badge = ch;
            //forgot the break in the first commit so it was wrong.
            break;
        }
        //in case we don't find something in the alphabet we return 'a' rather
        //than panic
        else {
            badge = 'a';
        }
    }
    //return the badge
    badge
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
