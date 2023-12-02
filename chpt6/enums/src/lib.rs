pub enum IpAddrKind {
    V4,
    V6,
}

#[test]
fn test_enum_values() {
    fn route(_ip_kind: IpAddrKind) -> bool {
        true
    }

    assert!(route(IpAddrKind::V4));
    assert!(route(IpAddrKind::V6));
}

// and also enums can be typed

pub enum ChessPieces {
    King(String),
    Queen(String),
    Rook(String),
    Knight(String),
    Bishop(String),
    Pawn(String),
}

#[test]
fn test_typed_enum_values() {
    let king = ChessPieces::King(String::from("K")); //its a function call!
    match king {
        ChessPieces::King(symbol) => assert_eq!(symbol, "K"),
        _ => panic!("Expected a King variant with the symbol \"K\""), // a little goofy, but...
    }
}

#[test]
fn test_option_enum() {
    let some_number = Some(5);
    assert_eq!(5, some_number.unwrap()); // more canononical is to use match {}
}
