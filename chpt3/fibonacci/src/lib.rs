pub fn nth_fib(n: isize) -> isize {
    // not very efficient but easy on the eyes!
    match n {
        1 => 1,
        2 => 1,
        _ => nth_fib(n - 1) + nth_fib(n - 2),
    }
}

pub fn nth_fib_non_recursive(n: isize) -> isize {
    if n <= 1 {
        return n;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 0..(n - 1) {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    return b;
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
    //#[case(49, 7_778_742_049)]   This takes forever! See the non-recursive solution.
    fn test_recursive_fib(#[case] input: isize, #[case] expected: isize) {
        assert_eq!(nth_fib(input), expected);
    }

    #[rstest]
    #[case(1, 1)]
    #[case(2, 1)]
    #[case(3, 2)]
    #[case(4, 3)]
    #[case(5, 5)]
    #[case(6, 8)]
    #[case(6, 8)]
    #[case(49, 7_778_742_049)]
    fn test_non_recursive_fib(#[case] input: isize, #[case] expected: isize) {
        assert_eq!(nth_fib_non_recursive(input), expected);
    }
}
