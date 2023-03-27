
#[cfg(test)]
mod tests {

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
    }
    return &s[..];
}

    #[test]
    fn test_index_of_first_word() {
        let s = String::from("Hello world!");
        let fw = first_word(&s);
        assert_eq!(fw, "Hello");
    }

    #[test]
    fn test_string_literals_as_slices() {
        let s: &str = "Hello, world!"; //immutable because &str is immutable reference
        assert_eq!(s, "Hello, world!")
    }

    #[test]
    fn test_string_slices_as_parameters() {
        // if you change the type of first_word from &String to &str, it works for either type
        let my_string = String::from("hello world");

        let world = first_word(&my_string[0..6]);
        assert_eq!(world, "hello");

        let word = first_word(&my_string[..]);
        assert_eq!(word, "hello");

        let word = first_word(&world);
        assert_eq!(word, "hello");

        let my_literal_string = "hello world";
        // first word works on slices of string literals too!
        assert_eq!(first_word(&my_literal_string[0..6]), "hello");
        assert_eq!(first_word(&my_literal_string[..]), "hello");

        //because string literals are string slices already,
        //this works too, without the slice syntax
        assert_eq!(first_word(my_literal_string), "hello");
    }

    #[test]
    fn test_other_slices() {
        //slices work on things other than strings
        let a = [1, 2, 3, 4, 5];
        let aslice = &a[1..3];
        assert_eq!(aslice, [2, 3]);
        assert_eq!(aslice, &[2, 3]); // the type of the slice is actually &[i32]
    }
}
