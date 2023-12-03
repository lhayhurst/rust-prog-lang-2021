use day2::parse_cube_sample_line;
use std::fs::File;
use std::io::{self, BufRead};

pub fn read_file_lines(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();
    lines
}

fn main() {
    let file_path = "aoc2023/day2/input.txt";
    match read_file_lines(file_path) {
        Ok(lines) => {
            let mut sum: i32 = 0;
            let mut power: i32 = 0;
            for line in lines {
                let result = parse_cube_sample_line(line.as_str());
                if result.has_valid_samples() == true {
                    sum = sum + result.game_num;
                }
                let lgcs = result.fewest_number_for_possible_game();
                power = power + lgcs.num_reds * lgcs.num_greens * lgcs.num_blues;
            }
            println!("Sum: {}", sum.to_string());
            println!("Power: {}", power.to_string());
        }
        Err(err) => {
            panic!("Unable to read file {}, reason: {}", file_path, err);
        }
    }
}
