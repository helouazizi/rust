#[allow(unused_imports)]
use roman_numbers::RomanDigit::*;
#[allow(unused_imports)]
use roman_numbers::RomanNumber;
fn main() {
    println!("{:?}", RomanNumber::from(32));
    // println!("{:?}", RomanNumber::from(9));
    // println!("{:?}", RomanNumber::from(45));
    // println!("{:?}", RomanNumber::from(0));
}

#[test]
fn it_works() {
    assert_eq!(RomanNumber::from(3).0, [I, I, I]);
    assert_eq!(RomanNumber::from(6).0, [V, I]);
    assert_eq!(RomanNumber::from(15).0, [X, V]);
    assert_eq!(RomanNumber::from(30).0, [X, X, X]);
    assert_eq!(RomanNumber::from(150).0, [C, L]);
    assert_eq!(RomanNumber::from(200).0, [C, C]);
    assert_eq!(RomanNumber::from(600).0, [D, C]);
    assert_eq!(RomanNumber::from(1500).0, [M, D]);
}

#[test]
fn substractive_notation() {
    assert_eq!(RomanNumber::from(4).0, [I, V]);
    assert_eq!(RomanNumber::from(44).0, [X, L, I, V]);
    assert_eq!(RomanNumber::from(3446).0, [M, M, M, C, D, X, L, V, I]);
    assert_eq!(RomanNumber::from(9).0, [I, X]);
    assert_eq!(RomanNumber::from(94).0, [X, C, I, V]);
}
