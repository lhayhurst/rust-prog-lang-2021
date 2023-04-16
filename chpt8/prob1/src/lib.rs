//Given a list of integers, use a vector and return the median (when sorted, the value in the
// middle position) and mode (the value that occurs most often; a hash map will be helpful here)
// of the list.
use std::collections::HashMap;

pub struct Stats {
    median: f32,
    mode: i32
}

fn get_median(sorted_numbers: Vec<i32>) -> f32 {
    let mid = sorted_numbers.len() / 2;
    let median: f32 = if sorted_numbers.len() % 2 == 0 {
        (sorted_numbers[mid - 1] + sorted_numbers[mid]) as f32 / 2.0
    } else {
        sorted_numbers[mid] as f32
    };
    median
}

fn get_mode(numbers: Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for num in numbers {
        *counts.entry(num).or_insert(0) += 1;
    }
    let mut mode = 0;
    let mut mode_count = 0;
    for (&num, &count) in &counts {
        if count > mode_count {
            mode = num;
            mode_count = count;
        }
    }
    mode
}

pub fn median_and_mode(numbers: Vec<i32>) -> Result<Stats, String> {
    if numbers.len() == 0 {
        return Err(String::from("empty vector!"));
    }

    let mut sorted: Vec<i32> = numbers.clone();
    sorted.sort();

    return Ok(Stats {
        median: get_median(sorted),
        mode: get_mode(numbers),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![1], 1)]
    #[case(vec![1, 2], 1)]
    #[case(vec![1, 2, 2], 2)]
    fn test_get_mode(#[case] input: Vec<i32>, #[case] expected: i32) {
        assert_eq!(get_mode(input), expected);
    }

    #[rstest]
    #[case(vec![1], 1.0)]
    #[case(vec![1, 2], 1.5)]
    #[case(vec![1, 2, 2], 2.0)]
    #[case(vec![1, 2, 3, 3], 2.5)]
    #[case(vec![1, 2, 3, 4, 6], 3.0)]
    fn test_get_median(#[case] input: Vec<i32>, #[case] expected: f32) {
        assert_eq!(get_median(input), expected);
    }


    #[test]
    fn it_handles_empty_list_ok() {
        let numbers = vec![];
        let result = median_and_mode(numbers);
        match result {
            Err(result) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn it_handles_single_entry_list_ok() {
        let numbers = vec![1];
        let result = median_and_mode(numbers);
        match result {
            Ok(result) => assert!(result.median == 1.0 && result.mode == 1),
            _ => assert!(false),
        }
    }

    #[test]
    fn it_handles_many_entry_list_ok() {
        let numbers = vec![1, 2];
        let result = median_and_mode(numbers);
        match result {
            Ok(result) => assert!(result.median == 1.5 && result.mode == 1),
            _ => assert!(false),
        }
    }

}
