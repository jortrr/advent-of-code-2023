#[macro_export]
macro_rules! test {
    ($expected:expr, $actual:expr) => {
        test!($expected, $actual, stringify!($actual));
    };

    // Match arm with format string parameter
    ($expected:expr, $actual:expr, $fmt:expr, $($arg:tt)*) => {
        let name = format!($fmt, $($arg)*);
        test!($expected, $actual, name);
    };

    ($expected:expr, $actual:expr, $name:expr) => {
        assert_eq!(
            $expected, $actual,
            "[Test Case] ❌ ({:?}, {:?})",
            $expected, $name
        );
        println!("[Test Case] ✅ ({:?}, {:?})", $expected, $name);
    };

}
