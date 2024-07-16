#[macro_export]
macro_rules! test {
    // For test cases that are not ran currently. The expected or actual value
    // can be added later, the symbol denotes that the test case is prohibited from running.
    // Makes life more convenient, when you know your test cases in advance, but have no values yet.
    ($actual:ident) => {
        println!("[Test Case] 🚫 ({:?}, {:?})", $actual, stringify!($actual));
    };

    ($expected:literal, $name:literal) => {
        println!("[Test Case] 🚫 ({:?}, {:?})", $expected, $name);
    };

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

#[macro_export]
macro_rules! debug {
    // Match arm with format string parameter
    (true, $fmt:expr, $($arg:tt)*) => {
        let message = format!($fmt, $($arg)*);
        debug!(true, message);
    };

    (true, $message:expr) => {
        dbg!("{}", $message);
    };

    (false, $fmt:expr, $($arg:tt)*) => {
        // Do nothing
    };

    (false, $message:expr) => {
        // Do nothing
    };

}
