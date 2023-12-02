#[cfg(test)]
mod tests {
    #[test]
    fn test_strcat() {
        let mut s = String::from("Hello");
        s.push_str(", world!");
        assert_eq!(s, "Hello, world!");
    }

    #[test]
    fn test_move_and_copy_semantics() {
        // integers don't act like references (pg 68)
        // because they implement the Copy trait
        // other types that implement the Copy trait, are
        // booleans, floats, characters, and tuples
        let x = 5;
        let mut y = x;

        y = y + 1;
        assert_eq!(x, 5);
        assert_eq!(y, 6);

        // but strings do
        let s1 = String::from("Hello");

        let mut s2 = s1.clone();
        s2.push_str(", world!");

        assert_eq!(s2, "Hello, world!");
        assert_eq!(s1, "Hello");
    }

    #[test]
    fn test_references_and_borrowing() {
        let s1 = String::from("Hello");

        fn calculate_len(s: &String) -> usize {
            s.len()
        }
        let len = calculate_len(&s1);
        assert_eq!(len, 5);
    }
}
