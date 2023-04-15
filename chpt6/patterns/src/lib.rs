#[test]
fn test_matching_with_option_t() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    assert_eq!(Some(6), plus_one(five));
    assert_eq!(None, plus_one(None));
}

#[test]
fn test_matching_with_catchall() {
    fn foo(x: u32) -> u32 {
        match x {
            3 => 5,
            7 => 9,
            _ => 13,
        }
    }
    assert_eq!(5, foo(3));
    assert_eq!(9, foo(7));
    assert_eq!(13, foo(0));
    assert_eq!(13, foo(13));
}
