#[test]
pub fn test_get_method_name() {
    fn approve() {
        let actual = ::get_method_name::get_method_name().ok();
        assert_eq!(Some("tests::test_get_method_name".to_string()), actual);
    }

    approve();
}
