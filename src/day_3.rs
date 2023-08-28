use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day_3(){
    let mut rucksack_tuple = ("_","_");
    let mut rucksack_sum = 0;
    // File rucksacks.txt must exist in the current path
    if let Ok(lines) = read_lines("./rucksacks.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(current_sack) = line {
                //keep track of the number of items in the current sack
                let mut item_count = 0;
                //variables to hold the two halves of the current sack
                let mut first_half = String::new();
                let mut second_half = String::new();
                    for ch in current_sack.chars(){
                        //split the current sack into two halves
                        if item_count < current_sack.len()/2 {
                            //push the characters into the sack halves
                            first_half.push(ch);
                        } else {
                            second_half.push(ch);
                        }
                        //increment the item count so we can split the sack in half
                        item_count += 1;
                    }
                    rucksack_tuple = (first_half.as_str(), second_half.as_str());
                    //This loop will look at each character and possibly count
                    //the same character twice if there are multiples in the
                    //the first rucksack and one in the second rucksack, that
                    //is why we break the loop after the first match.
                    for ch in rucksack_tuple.0.chars() {
                        if rucksack_tuple.1.contains(ch) {
                            if ch.is_uppercase() {
                                rucksack_sum += ch as u32 - 38;
                                break;
                            } else {
                                rucksack_sum += ch as u32 - 96;
                                break;
                            }
                        }
                    }
                }
            }
        }
    println!("Total sum of rucksacks is {}", rucksack_sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
