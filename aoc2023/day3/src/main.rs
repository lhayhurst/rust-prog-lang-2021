use day3::EngineSchematic;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn read_file_lines(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();
    lines
}

fn main() {
    let file_path = "aoc2023/day3/input.txt";

    match read_file_lines(file_path) {
        Ok(lines) => {
            let schema = EngineSchematic::create_schema(lines);
            let engine_schematic = EngineSchematic { schema };
            let part_numbers = engine_schematic.get_part_numbers();
            println!("{}", part_numbers.len());
            println!("Sum: {}", engine_schematic.sum_part_numbers());
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}
