#![feature(plugin)]
#![plugin(roman_numerals)]

#[test]
fn test() {
    assert_eq!(rn!(MMXV), 2015);
}
