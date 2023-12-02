use std::collections::HashMap;

pub fn create_number_map() -> HashMap<&'static str, &'static str> {
    let mut number_map = HashMap::new();
    number_map.insert("one", "o1e");
    number_map.insert("two", "t2o");
    number_map.insert("three", "t3e");
    number_map.insert("four", "4");
    number_map.insert("five", "f5e");
    number_map.insert("six", "6");
    number_map.insert("seven", "s7n");
    number_map.insert("eight", "e8t");
    number_map.insert("nine", "n9e");
    number_map
}

pub fn prepass_substitute(s: &str, map: HashMap<&'static str, &'static str>) -> String {
    let mut ret = s.to_string();
    for (key, val) in map.iter() {
        ret = ret.replace(key.clone(), val);
    }
    return ret;
}

#[cfg(test)]
mod test_prepass_substitute {
    use super::*;

    //Zero case
    #[test]
    fn test_can_handle_empty_string() {
        let result = prepass_substitute("", create_number_map());
        assert_eq!("", result);
    }

    #[test]
    fn test_can_handle_simplest_case() {
        let result = prepass_substitute("one", create_number_map());
        assert_eq!("o1e", result);
    }

    #[test]
    fn test_can_handle_trickier_case() {
        let result = prepass_substitute("eighthree", create_number_map());
        assert_eq!("e8t3e", result);
    }
}

pub fn get_numbers_from_string(s: &str) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    let num_subbed_str = prepass_substitute(s, create_number_map());
    for c in num_subbed_str.to_string().chars() {
        if c.is_digit(10) {
            numbers.push(c.to_digit(10).unwrap() as i32);
        }
    }
    return numbers;
}

#[cfg(test)]
mod test_numbers_from_string {
    use super::*;

    //Zero case
    #[test]
    fn test_can_get_number_from_empty_string() {
        let result = get_numbers_from_string("foo");
        assert_eq!(0, result.len());
    }

    //One case
    #[test]
    fn test_can_get_number_from_string_with_single_number() {
        let result = get_numbers_from_string("foo1");
        assert_eq!(1, result.len());
        assert_eq!(1, result[0]);
    }

    //Many case
    #[test]
    fn test_can_get_number_from_string() {
        let result = get_numbers_from_string("1foo3a");
        assert_eq!(2, result.len());
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 3);
    }

    #[test]
    fn test_can_get_number_from_weird_edge_case_string() {
        let result = get_numbers_from_string("eighthree");
        assert_eq!(2, result.len());
        assert_eq!(result[0], 8);
        assert_eq!(result[1], 3);
    }
}

#[derive(Debug)]
pub enum CalibrationExtractError {
    NoNumbersInCalibrationString,
}

pub fn get_calibration_value(numbers: Vec<i32>) -> Result<i32, CalibrationExtractError> {
    if numbers.is_empty() {
        Err(CalibrationExtractError::NoNumbersInCalibrationString)
    } else {
        Ok(numbers[0] * 10 + *numbers.last().unwrap_or(&0))
    }
}

#[cfg(test)]
mod test_get_calibration_value {
    use super::*;

    //Zero case
    #[test]
    fn test_can_get_value_from_empty_vec() {
        let result = get_calibration_value(vec![]);
        assert!(result.is_err());
    }

    //One case
    #[test]
    fn test_can_get_value_from_single_value_vec() {
        let result = get_calibration_value(vec![1]);
        assert_eq!(11, result.unwrap());
    }

    //N case
    #[test]
    fn test_can_get_value_from_n_value_vec() {
        let result = get_calibration_value(vec![1, 2, 3]);
        assert_eq!(13, result.unwrap());
    }
}
