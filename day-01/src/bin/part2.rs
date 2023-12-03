use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Digit {
    pub digit: i32,
    pub index: usize,
    pub calibration: String,
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
    
    let spelled_digit = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    let mut sum: i32 = 0;

    for line in data.lines() {
        let mut digits = Vec::new();

        for (key, val) in spelled_digit.iter() {

            for (index, _occurence) in line.match_indices(key) {
                digits.push(Digit {
                    digit: *val,
                    index: index,
                    calibration: line.to_string(),
                })
            }
        }

        for (index, character) in line.chars().enumerate() {
            if character.is_digit(10) {
                digits.push(Digit {
                    digit: character.to_digit(10).unwrap() as i32,
                    index: index,
                    calibration: line.to_string()
                });
            }
        }

        digits.sort_by_key(|digit| digit.index);

        for element in digits.iter() {
            println!("{:?}", element);
        }
        
        let value_str = format!("{}{}", digits[0].digit, digits[digits.len() - 1].digit);
        let value_i32: i32 = value_str.parse().unwrap();
        // println!("{} = {}", line, value_i32);
        sum = sum + value_i32;
    }
    println!("{}", sum);
}
