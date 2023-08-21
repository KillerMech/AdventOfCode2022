use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

pub fn day_2_2(){

    let mut total_score = 0;
    let mut first_char:char = '_';
    let mut second_char:char=  '_';
    let mut char_iterator:u32 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(key) = line {
                // Iterate over the chars in the string
                for ch in key.chars(){
                    // Keep track of whether we are on the first char or second
                    char_iterator = char_iterator + 1;
                    // If the char is a space, ignore
                    if ch == ' ' {continue;}
                    // Put the first char in first_char
                    if char_iterator == 1 {first_char = ch;}
                    // Put the second char in second_char
                    else {second_char = ch;}
                }
                char_iterator = 0;
                // What to do if opponent used rock?
                if first_char == 'A'{
                    if second_char == 'X' {total_score = total_score + 3;}
                    else if second_char == 'Y' {total_score = total_score + 4;}
                    else if second_char == 'Z' {total_score = total_score + 8;}
                }
                // Paper?
                else if first_char == 'B'{
                    if second_char == 'X' {total_score = total_score + 1;}
                    else if second_char == 'Y' {total_score = total_score + 5;}
                    else if second_char == 'Z' {total_score = total_score + 9;}
                }
                // Scissors?
                else if first_char == 'C'{
                    if second_char == 'X' {total_score = total_score + 2;}
                    else if second_char == 'Y' {total_score = total_score + 6;}
                    else if second_char == 'Z' {total_score = total_score + 7;}
                }
            }
        }
    }
    println!("Your total score is: {}", total_score);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
