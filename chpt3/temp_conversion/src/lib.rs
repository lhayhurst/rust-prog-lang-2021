pub fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}

pub fn celsius_to_fahrenheit(temp: f64) -> f64 {
    (temp * 9.0 / 5.0 ) + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;
    use rstest::rstest;

    #[rstest]
    #[case(-459.67, -273.15)] //absolute zero
    #[case(32.0, 0.00)] // freezing/melting point
    #[case(69.8, 21.0)] // room temp
    #[case(98.6, 37.00)] // average body temp
    #[case(212.0, 100.0)] // boiling point of water
    fn test_fahrenheit_to_celsius(#[case] input: f64, #[case] expected: f64) {
        assert_approx_eq!(f64, fahrenheit_to_celsius(input), expected, epsilon = 0.01)
    }

    #[rstest]
    #[case(-273.15, -459.67)] //absolute zero
    #[case(0.00, 32.00)] // freezing/melting point
    #[case(21.0, 69.8)] // room temp
    #[case(37.00, 98.60)] // average body temp
    #[case(100.0, 212.0)] // boiling point of water
    fn test_celsius_to_fahrenheit(#[case] input: f64, #[case] expected: f64) {
        assert_approx_eq!(f64, celsius_to_fahrenheit(input), expected, epsilon = 0.01)
    }


}
