pub fn nth_fib(n: isize) -> isize {
    // not very efficient but easy on the eyes!
    match n {
        1 => 1,
        2 => 1,
        _ => nth_fib(n - 1) + nth_fib(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 1)]
    #[case(2, 1)]
    #[case(3, 2)]
    #[case(4, 3)]
    #[case(5, 5)]
    #[case(6, 8)]
    fn test_fib(#[case] input: isize, #[case] expected: isize) {
        assert_eq!(nth_fib(input), expected);
    }
}
