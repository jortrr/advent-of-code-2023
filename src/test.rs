pub fn test<T: std::cmp::PartialEq + std::fmt::Debug>(expected: &T, actual: &T) {
    assert_eq!(
        expected, actual,
        "Test case failed: this value should always equal '{:?}'.",
        expected
    );
}

pub fn test_and_debug<T: std::cmp::PartialEq + std::fmt::Debug>(
    expected: &T,
    actual: &T,
    message: &str,
) {
    test(expected, actual);
    println!("[Test Succeeded] {}", message);
}
