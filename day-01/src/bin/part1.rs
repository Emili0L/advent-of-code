use std::fs;

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
    
    let mut sum: i32 = 0;

    for line in data.lines() {
        let mut digits = Vec::new();
        for character in line.chars() {
            if character.is_digit(10) {
                digits.push(character);
            }
        }
        let value_str = format!("{}{}", digits[0], digits[digits.len() - 1]);
        let value_i32: i32 = value_str.parse().unwrap();
        println!("{}", value_i32);
        sum = sum + value_i32;
    }
    println!("{}", sum);
}
