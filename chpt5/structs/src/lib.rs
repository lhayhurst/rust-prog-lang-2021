#[cfg(test)]
mod tests {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    #[test]
    fn test_make_simple_mutable_user() {
        let mut u1 = User {
            active: true,
            username: String::from("inigomontoya"),
            email: String::from("inigo@montoya.com"),
            sign_in_count: 1010,
        };

        assert_eq!(u1.active, true);
        assert_eq!(u1.username, "inigomontoya");
        assert_eq!(u1.email, "inigo@montoya.com");
        assert_eq!(u1.sign_in_count, 1010);

        u1.sign_in_count = 1111;
        assert_eq!(u1.sign_in_count, 1111);
    }

    #[test]
    fn test_can_make_user_using_factory_function() {
        fn build_user(email: String, username: String) -> User {
            User {
                active: true,
                username: username,
                email: email,
                sign_in_count: 1,
            }
        }

        let u1 = build_user(
            String::from("inigo@montoya.com"),
            String::from("inigomontoya"),
        );
        assert_eq!(u1.active, true);
        assert_eq!(u1.username, "inigomontoya");
        assert_eq!(u1.email, "inigo@montoya.com");
        assert_eq!(u1.sign_in_count, 1);
    }

    #[test]
    fn test_field_init_short_hand() {
        fn build_user(email: String, username: String) -> User {
            User {
                active: true,
                username,
                email,
                sign_in_count: 1,
            }
        }

        let u1 = build_user(
            String::from("inigo@montoya.com"),
            String::from("inigomontoya"),
        );
        assert_eq!(u1.active, true);
        assert_eq!(u1.username, "inigomontoya");
        assert_eq!(u1.email, "inigo@montoya.com");
        assert_eq!(u1.sign_in_count, 1);
    }

    #[test]
    fn test_create_instance_from_other_instance() {
        let u1 = User {
            active: true,
            username: String::from("wesley"),
            email: String::from("wesley@farmboy.com"),
            sign_in_count: 1010,
        };

        let u2 = User {
            email: String::from("dread@pirateroberts.com"),
            ..u1
        };

        assert_eq!(u2.active, true);
        assert_eq!(u2.username, "wesley");
        assert_eq!(u2.email, "dread@pirateroberts.com");
        assert_eq!(u2.sign_in_count, 1010);
    }

    #[test]
    fn test_can_make_tuple_structs_without_named_fields() {
        struct Point(i32, i32, i32);
        let origin = Point(1, 10, 100);
        assert_eq!(origin.0, 1);
        assert_eq!(origin.1, 10);
        assert_eq!(origin.2, 100);
    }

    #[test]
    fn test_can_add_useful_functionality_with_eq_trait() {
        #[derive(Debug)]
        enum BookFormat {
            Paperback,
            Hardback,
            Ebook,
        }

        #[derive(Debug)]
        struct Book {
            isbn: i64,
            format: BookFormat,
        }

        impl PartialEq for Book {
            fn eq(&self, other: &Self) -> bool {
                self.isbn == other.isbn
            }
        }
        impl Eq for Book {}

        let ebook = Book {
            isbn: 9780156035217,
            format: BookFormat::Ebook,
        };

        let hardback = Book {
            isbn: 9780156035217,
            format: BookFormat::Hardback,
        };

        assert_eq!(ebook, hardback);
        assert!(ebook == hardback); // == works as well

        let another_book = Book {
            isbn: 1840228304,
            format: BookFormat::Paperback,
        };

        assert!(ebook != another_book)
    }

    #[test]
    fn test_can_add_methods_to_struct() {
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }

            fn width(&self) -> bool {
                self.width > 0
            }
        }

        let r = Rectangle {
            width: 10,
            height: 20,
        };
        assert_eq!(r.area(), 10 * 20);
        assert!(r.width());

        let r2 = Rectangle {
            width: 0,
            height: 20,
        };
        assert!(r2.width() == false);
    }

    #[test]
    fn test_associated_function() {
        // functions in an impl block are called associated functions because
        // they are associated with whatever the type is after the impl
        // but they don't have to take self as an argument
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn square(size: u32) -> Self {
                Self {
                    width: size,
                    height: size,
                }
            }
        }

        // you can have multiple implementation blocks if you want, see page 102
        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
        }

        let square = Rectangle::square(10);
        assert!(square.width == square.height);
        assert_eq!(square.area(), 100);
    }
}
