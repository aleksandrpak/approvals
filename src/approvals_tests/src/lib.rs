#![feature(plugin, tempdir)]
#![plugin(approvals)]
#[macro_use] extern crate approvals;

#[test]
fn test_123() {
    let actual = "test123";
    approve_file!(actual, expected_file);
}
