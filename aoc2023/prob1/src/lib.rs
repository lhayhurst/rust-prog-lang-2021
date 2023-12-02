pub fn get_numbers_from_string(s: &str) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    for c in s.chars() {
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

