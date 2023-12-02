use std::fs::File;
use std::io::{self, BufRead};
use prob1_and_2::get_calibration_value;
use prob1_and_2::get_numbers_from_string;

pub fn read_file_lines(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();
    lines
}

fn main() {
    let file_path = "aoc2023/day1/input.txt";
    match read_file_lines(file_path) {
        Ok(lines) => {
            let mut sum: i32 = 0;
            for line in lines {
                let result = get_calibration_value(get_numbers_from_string(line.as_str()));
                match result {
                    Err(_err) => {
                        panic!(
                            "Received error from invalid line input from the elves: {}",
                            line
                        );
                    }
                    Ok(value) => {
                        sum = sum + value;
                        println!("{} -> {} ({})", line, result.unwrap(), sum);
                    }
                }
            }
            println!("{}", sum.to_string());
        }
        Err(err) => {
            panic!("Unable to read file {}, reason: {}", file_path, err);
        }
    }
}
