use std::{fs, char, borrow::BorrowMut};

#[derive(Debug)]
pub struct PartNumber {
    value: i32,
    start_index: usize,
    end_index: usize,
}

fn main() {
    let file_path = "./src/bin/input.txt";
    let data = match fs::read_to_string(file_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file '{}': {:?}",
                      "Error:", file_path, e);
            std::process::exit(1);
        }
    };

    let lines = data.lines().enumerate();
    let mut lines_access: Vec<&str> = data.lines().collect();
    let mut sum = 0;


    // splitting and then searching vs. checking each char???? whats more expensive

    for (line_index, line) in lines {


        let mut part_numbers: Vec<PartNumber> = Vec::new();
        let mut current_number = String::new();
        let mut previous: char = '.';
        let mut last_index: usize = 0;

        let characters = line.char_indices().into_iter();
        

        for (character_index, character) in characters {
            //print!("{}",previous);
            if character.is_digit(10) {
                if previous.is_digit(10) {
                    current_number.push(character);
                    last_index = character_index;
                }
                else {
                    if current_number.len() == 0 {
                        current_number = character.to_string();
                        last_index = character_index;
                    }
                    else {
                        //println!("Number: {} Length:{}",current_number, current_number.len());
                        part_numbers.push(PartNumber {
                            value: current_number.parse().unwrap_or(0),
                            start_index: last_index + 1 - current_number.len(),
                            end_index: last_index,
                        });
                        current_number = character.to_string();
                        last_index = character_index;
                    }
                }
            }
            previous = character;
        }
        if current_number.len() > 0 {
            part_numbers.push(PartNumber {
                value: current_number.parse().unwrap_or(0),
                start_index: last_index + 1 - current_number.len(),
                end_index: last_index,
            });
        }

        for part in part_numbers.iter() {
            println!("Line: {}, Value: {} Start: {} End: {}", line_index, part.value, part.start_index, part.end_index);
        }

        for part in part_numbers.iter() {
            // init
            let mut left: usize = 0; 
            if part.start_index > 0 {
                left = part.start_index - 1;
            }
            else {
                left = 0; 
            }
            let mut right = part.end_index + 2;
            println!("Line: {}, Value: {}, Start_Index: {}, End_Index: {}, Right: {}", line_index, part.value, part.start_index, part.end_index, right);
            if right > line.len() - 1 {
                right = right - (right - line.len() + 1);
            }
            println!("Right: {}, Line-Length: {}", right, line.len());
            
            // test

            if line_index > 0 {
                let previous_line = lines_access[line_index - 1];
                let top_chars = &previous_line[left..right];
                println!("Top:{}", top_chars);
            }

            let current_line: Vec<char> = lines_access[line_index].chars().collect();

            println!("Mid:{}{}{}", current_line[left], part.value, current_line[right]);

            if line_index < lines_access.len() - 1 {
                let next_line = lines_access[line_index + 1];
                let bot_chars = &next_line[left..right];
                println!("Bot:{}", bot_chars);
            }
            

            // sum

            
            if line_index > 0 {
                let previous_line = lines_access[line_index - 1];
                let top_chars = &previous_line[left..right];
                if top_chars.chars().any(|c| c.is_ascii_punctuation() && c != '.') {
                    sum = sum + part.value;
                    continue;
                }
            }

            let current_line: Vec<char> = lines_access[line_index].chars().collect();

            if part.start_index > 0 {
                if current_line[left].is_ascii_punctuation() && current_line[left] != '.' {
                    sum = sum + part.value;
                    continue;
                }
            }
            if current_line[right-1].is_ascii_punctuation() && current_line[right-1] != '.' {
                sum = sum + part.value;
                continue;
            }

            if line_index < lines_access.len() - 1 {
                let next_line = lines_access[line_index + 1];
                let bot_chars = &next_line[left..right];
                if bot_chars.chars().any(|c| c.is_ascii_punctuation() && c != '.') {
                    sum = sum + part.value;
                    continue;
                }
            }
            println!("Line: {}, Value: {}. No valid part number", line_index, part.value)
        }
    }    
    println!("Sum: {}", sum);
}
