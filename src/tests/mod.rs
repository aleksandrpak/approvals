#[test]
pub fn test_get_method_name() {
    let actual = ::get_method_name::get_method_name(0).ok();
    assert_eq!(Some("tests::test_get_method_name".to_string()), actual);
}

#[test]
pub fn test_get_method_name_invalid_level() {
    match ::get_method_name::get_method_name(9) {
        Err(::get_method_name::Error::InvalidLevel) => {}
        Err(err) => assert!(false, "Expected invalid level but was error: {:?}", err),
        Ok(ok) => assert!(false, "Expected invalid level but was ok: {}", ok),
    }
}

#[test]
pub fn test_get_method_name_no_method_separator() {
    match ::get_method_name::get_method_name(3) {
        Err(::get_method_name::Error::NoMethodSeparator) => {}
        Err(err) => assert!(false, "Expected invalid level but was error: {:?}", err),
        Ok(ok) => assert!(false, "Expected invalid level but was ok: {}", ok),
    }
}

#[test]
pub fn test_get_dir() {
    // TODO: make returing Result<> and check all possible outcomes
    ::get_dir("test_dir");

    ::std::fs::remove_dir(::std::path::Path::new("test_dir")).unwrap();
}

#[test]
pub fn test_get_file_contents() {
    // TODO: make returing Result<> and check all possible outcomes
    assert_eq!("Approvals!\n", ::get_file_contents("contents", "src/tests"));
}

// TODO: write test for 'write_actual'
// TODO: write test for launching diff tool

#[test]
fn test_approve() {
    ::approve("This is approved!");
}

// TODO: fix work of more than one approve
// #[test]
// fn test_approve_twice() {
//     ::approve("This is approved twice!");
// }
