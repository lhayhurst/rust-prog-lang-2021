#[test]
fn test_string_create() {
    let s = String::new();
    assert_eq!(0, s.len());
}

#[test]
fn test_make_string_from_literal() {
    let data = "initial contents";
    let s = data.to_string();
    assert_eq!(s, data);
}

#[test]
fn test_updating_a_string() {
    let mut s = String::from("foo");
    s.push_str("bar");
    assert_eq!(s, "foobar");
}

#[test]
fn test_append_character_to_a_string() {
    let mut s = String::from("lo");
    s.push('l');
    assert_eq!(s, "lol");
}

#[test]
fn test_str_concat() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    assert_eq!(s3, "Hello, world!");
}

#[test]
fn test_str_concat_via_format() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = format!("{s1}-{s2}-{s3}");
    assert_eq!(s4, "tic-tac-toe");
}
