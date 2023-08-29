use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day_3_2(){
    let mut elf_rucksacks: [String; 3];
    let mut elf_counter = 0;
    let mut rucksack_sum = 0;
    // File rucksacks.txt must exist in the current path
    if let Ok(lines) = read_lines("./rucksacks.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let mut rucksack_return = '_';
            if let Ok(current_sack) = line {
                    let current_string = current_sack.to_string();
                    elf_rucksacks[elf_counter] = current_string;
                    elf_counter += 1;
                    if elf_counter == 3 {
                        elf_counter = 0;
                        rucksack_return = rucksack_badge(&mut elf_rucksacks);
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

fn rucksack_badge(rucksack: &mut [&str]) -> char {
    let mut rucksacks = rucksack;
    let badge: char;

    for ch in rucksacks[0].chars() {
        if rucksacks[1].contains(ch) && rucksacks[2].contains(ch) {
            badge = ch;
        }
    }
    badge
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
