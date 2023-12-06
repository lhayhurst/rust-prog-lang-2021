use std::collections::HashSet;

pub struct EngineSchematic {
    schema: Vec<Vec<char>>,
}

fn str_to_int32(input: &str) -> Option<i32> {
    match input.parse::<i32>() {
        Ok(parsed_number) => Some(parsed_number),
        Err(_) => None,
    }
}

impl EngineSchematic {
    pub fn new(schema: Vec<Vec<char>>) -> EngineSchematic {
        Self { schema }
    }

    pub fn create_schema(input: Vec<&str>) -> Vec<Vec<char>> {
        input
            .iter()
            .map(|line| line.trim().chars().collect())
            .collect()
    }

    pub fn get_part_numbers(&self) -> Vec<i32> {
        let mut numbers = HashSet::new();
        for (row_num, row) in self.schema.iter().enumerate() {
            for (col_num, col_val) in row.iter().enumerate() {
                if col_val.is_digit(10) {
                    if self.is_adjacent_to_symbol(row_num, col_num) {
                        if let Some(extracted_number) =
                            self.extract_number_from_location(row_num, col_num)
                        {
                            numbers.insert(extracted_number);
                        }
                    }
                }
            }
        }
        return numbers.iter().cloned().collect();
    }

    pub fn extract_number_from_location(&self, row: usize, column: usize) -> Option<i32> {
        // start at the middle and go left, then go right
        if self.schema[row][column].is_digit(10) {
            let mut ret = self.schema[row][column].to_string();
            let mut curr_col = column as i32 - 1;

            //collect left
            while curr_col >= 0 && self.schema[row][curr_col as usize].is_digit(10) {
                ret = self.schema[row][curr_col as usize].to_string() + &*ret;
                curr_col -= 1;
            }
            curr_col = column as i32 + 1;

            let num_cols = self.schema[row].len() as i32;
            //collect right
            while curr_col < num_cols && self.schema[row][curr_col as usize].is_digit(10) {
                ret = ret + &*self.schema[row][curr_col as usize].to_string();
                curr_col += 1;
            }
            return str_to_int32(ret.as_str());
        }
        None
    }

    pub fn is_symbol(&self, row: i32, column: i32) -> bool {
        //check to see if outside the boundaries of the schema
        if row < 0 {
            return false;
        }

        if row >= self.schema.len() as i32 {
            return false;
        }

        if column as usize >= self.schema[row as usize].len() {
            return false;
        }

        if column < 0 {
            return false;
        }

        let ch = self.schema[row as usize][column as usize];
        if ch.is_digit(10) || ch == '.' {
            return false;
        }
        true
    }

    pub fn is_adjacent_to_symbol(&self, row: usize, column: usize) -> bool {
        //look left
        if self.is_symbol(row as i32, column as i32 - 1) {
            return true;
        }

        //look right
        if self.is_symbol(row as i32, column as i32 + 1) {
            return true;
        }
        //look up
        if self.is_symbol(row as i32 - 1, column as i32) {
            return true;
        }

        //look diagonal up, right
        if self.is_symbol(row as i32 - 1, column as i32 + 1) {
            return true;
        }

        //look diagonal up, left
        if self.is_symbol(row as i32 - 1, column as i32 - 1) {
            return true;
        }

        //look diagonal down, right
        if self.is_symbol(row as i32 + 1, column as i32 + 1) {
            return true;
        }

        //look diagonal down, left
        if self.is_symbol(row as i32 + 1, column as i32 - 1) {
            return true;
        }

        return false;
    }
}

mod test_can_create_schema {
    use crate::EngineSchematic;

    fn get_test_schematic() -> EngineSchematic {
        let input = r#"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        "#;
        let trimmed_input = input.trim();
        let lines: Vec<&str> = trimmed_input.trim().lines().collect();
        let schema = EngineSchematic::create_schema(lines);
        let engine_schematic = EngineSchematic { schema };
        engine_schematic
    }

    #[test]
    fn test_can_extract_number_from_location() {
        let es = get_test_schematic();

        assert_eq!(None, es.extract_number_from_location(0, 4));

        assert_eq!(467, es.extract_number_from_location(0, 0).unwrap());
        assert_eq!(467, es.extract_number_from_location(0, 0).unwrap());
        assert_eq!(467, es.extract_number_from_location(0, 0).unwrap());

        assert_eq!(35, es.extract_number_from_location(2, 3).unwrap());
        assert_eq!(35, es.extract_number_from_location(2, 3).unwrap());

        assert_eq!(664, es.extract_number_from_location(9, 2).unwrap());
        assert_eq!(598, es.extract_number_from_location(9, 7).unwrap());
    }

    #[test]
    fn test_is_adjacent_to_symbol() {
        let es = get_test_schematic();
        assert_eq!(false, es.is_adjacent_to_symbol(0, 0));
        assert_eq!(true, es.is_adjacent_to_symbol(0, 2));
        assert_eq!(true, es.is_adjacent_to_symbol(4, 2));
        assert_eq!(true, es.is_adjacent_to_symbol(6, 4));
    }

    #[test]
    fn test_is_symbol() {
        let es = get_test_schematic();
        assert_eq!(false, es.is_symbol(0, 0));
        assert_eq!(true, es.is_symbol(1, 3));
        assert_eq!(true, es.is_symbol(8, 3));
        assert_eq!(true, es.is_symbol(8, 5));
        assert_eq!(false, es.is_symbol(-1, 4));
        assert_eq!(false, es.is_symbol(0, 100));
    }

    #[test]
    fn test_get_part_numbers() {
        let es = get_test_schematic();
        let part_numbers = es.get_part_numbers();
        assert_eq!(part_numbers.len(), 8);
        assert_eq!(true, part_numbers.contains(&467));
        assert_eq!(true, part_numbers.contains(&35));
        assert_eq!(true, part_numbers.contains(&633));
        assert_eq!(true, part_numbers.contains(&617));
        assert_eq!(true, part_numbers.contains(&598));
        assert_eq!(true, part_numbers.contains(&592));
        assert_eq!(true, part_numbers.contains(&755));
        assert_eq!(true, part_numbers.contains(&664));
    }
}
