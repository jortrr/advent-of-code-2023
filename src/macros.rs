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

    ($expected:literal, $actual:expr, $name:expr) => {
        assert_eq!(
            $expected, $actual,
            "[Test Case] ❌ ({:?}, {:?})",
            $expected, $name
        );
        println!("[Test Case] ✅ ({:?}, {:?})", $expected, $name);
    };

    ($expected:expr, $actual:expr, $name:expr) => {
        assert_eq!(
            $expected, $actual,
            "[Test Case] ❌ ({:?}, {:?})",
            $expected, $name
        );
        println!("[Test Case] ✅ ({:?}, {:?})", stringify!($expected), $name);
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

// See: https://chatgpt.com/share/d866e424-9d25-441f-a232-bf78c8372d7c
#[macro_export]
macro_rules! define_convertable_enum {
    ($name:ident { $($variant:ident $(($opt:ty))? => $char:expr),* $(,)? }) => {
        #[derive(Debug)]
        enum $name {
            $($variant$(($opt))?),*
        }

        impl $name {
            fn from_char(c: char) -> $name {
                match c {
                    $(
                        $char => $name::$variant$(($opt::default()))?,
                    )*
                    _ => panic!("Invalid {} char: '{}'.", stringify!($name), c),
                }
            }

            fn to_char(&self) -> char {
                match self {
                    $(
                        $name::$variant$(($opt::default()))? => $char,
                    )*
                }
            }
        }
    };
}
