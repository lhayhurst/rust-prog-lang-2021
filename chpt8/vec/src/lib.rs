#[test]
fn test_create_empty_vec() {
    let mut v: Vec<i32> = Vec::new();
    assert_eq!(v.len(), 0);
}

#[test]
fn test_vec_push_and_pop() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    assert_eq!(v.len(), 1);

    let one = v.pop();
    assert_eq!(one, Some(1));
}

#[test]
fn test_read_elements_from_vec() {
    let v = vec![1, 2, 3, 4, 5];

    // you can access it using the traditional [] array access
    let third: &i32 = &v[2];
    assert_eq!(*third, 3);

    // alternatively you can use get()
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_vector_iteration() {
    let v = vec![1, 2, 3, 4, 5];
    let mut i = 1;
    for j in &v {
        assert_eq!(i, *j);
        i += 1;
    }
}

#[test]
fn test_changing_vector_contents_during_iteration() {
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i *= 3;
    }

    let mut k = 1;
    for j in &v {
        assert_eq!(*j, k * 3);
        k += 1;
    }
}
