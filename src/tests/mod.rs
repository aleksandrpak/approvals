#[test]
pub fn test_get_method_name() {
    fn approve() {
        assert_eq!(Some("tests::test_get_method_name".to_string()), ::get_method_name());
    }

    approve();
}
