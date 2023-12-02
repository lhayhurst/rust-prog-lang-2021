use std::collections::HashMap;

#[test] 
fn test_create_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    assert!(scores.contains_key("Blue"));
}

#[test]
fn test_get_from_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);  //awwwwwkward
    assert_eq!(score, 10);

    let nteam_name = String::from("Red");
    let non_existing_score = scores.get(&nteam_name).copied().unwrap_or(0);
    assert_eq!(non_existing_score, 0);
}

#[test]
fn test_overwrite_value_in_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    let score = scores.get("Blue").copied().unwrap_or(0);
    assert_eq!(score, 25); //overwritten value
}

#[test]
fn test_can_add_key_and_value_but_only_if_key_is_not_present() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    let score = scores.get("Yellow").copied().unwrap_or(0);
    assert_eq!(score, 50);
    scores.entry(String::from("Blue")).or_insert(50);
    let score = scores.get("Yellow").copied().unwrap_or(0);
    assert_eq!(score, 50); //overwritten value
}

#[test]
fn test_can_update_a_value_based_on_old_value() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    assert_eq!(map.get("hello").copied().unwrap_or(0), 1);
    assert_eq!(map.get("world").copied().unwrap_or(0), 2);
    assert_eq!(map.get("wonderful").copied().unwrap_or(0), 1);
}