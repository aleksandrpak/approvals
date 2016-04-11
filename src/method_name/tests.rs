use method_name;
use method_name::error::Error;

// TODO: Fix travis CI where there is nothing after actual method name
// #[test]
// pub fn test_get_method_name() {
//     let actual = method_name::get(0).ok();
//     assert_eq!(Some("approvals::method_name::tests::test_get_method_name".to_string()), actual);
// }

#[test]
pub fn test_get_method_name_invalid_level() {
    match method_name::get(9) {
        Err(Error::InvalidLevel) => {}
        Err(err) => panic!("Expected invalid level but was error: {:?}", err),
        Ok(ok) => panic!("Expected invalid level but was ok: {}", ok),
    }
}

#[test]
pub fn test_get_method_name_no_method_separator() {
    match method_name::get(3) {
        Err(Error::NoMethodSeparator) => {}
        Err(err) => panic!("Expected no method separator but was error: {:?}", err),
        Ok(ok) => panic!("Expected no method separator but was ok: {}", ok),
    }
}
