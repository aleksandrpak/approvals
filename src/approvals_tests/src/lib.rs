#![feature(plugin)]
#![plugin(approvals)]

#[test]
fn test_123() {
    assert_eq!(rn!(MMXV), 2015);
}
