use wasm_utils::utils::UtilsHandler;

#[test]
fn test_format_number() {
    let number = 12234345.23456;
    let result = UtilsHandler::format_float(number, None);
    assert_eq!(result, "12,234,345.23456");

    let result = UtilsHandler::format_float(number, Some(3));
    assert_eq!(result, "12,234,345.235");

    let number = 12234345;
    let result = UtilsHandler::format_integer(number);
    assert_eq!(result, "12,234,345");

    let number = -12234345;
    let result = UtilsHandler::format_integer(number);
    assert_eq!(result, "-12,234,345");
}

#[test]
fn test_capitalize_first_char() {
    let str = "test1234567";
    let result = UtilsHandler::capitalize_first_char(str);
    assert_eq!(result, "Test1234567");
}

#[test]
fn test_hump_with_line() {
    let str = "testItemManager";
    let result = UtilsHandler::hump_with_line(str, Some('-'));
    assert_eq!(result, "test-item-manager");
}

#[test]
fn test_format_phone() {
    let str = "13200000000";
    let result = UtilsHandler::format_phone(str, None).unwrap();
    assert_eq!(result, "132 0000 0000");

    let result = UtilsHandler::format_phone(str, Some('-')).unwrap();
    assert_eq!(result, "132-0000-0000");
}
