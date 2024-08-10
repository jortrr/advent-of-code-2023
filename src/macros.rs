#[macro_export]
macro_rules! test {
    // For test cases that are not ran currently. The expected or actual value
    // can be added later, the symbol denotes that the test case is prohibited from running.
    // Makes life more convenient, when you know your test cases in advance, but have no values yet.
    ($actual:ident) => {
        assert!(false,"[Test Case] 🚫 ({:?}, {:?})", $actual, stringify!($actual));
    };

    ($condition:expr) => {
        assert!(
            $condition,
            "[Test Case] ❌ ({:?})",
            stringify!($condition)
        );
        println!("[Test Case] ✅ ({:?})", stringify!($condition));
    };

    ($expected:literal, $name:literal) => {
        assert!(false,"[Test Case] 🚫 ({:?}, {:?})", $expected, $name);
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
        println!("[Test Case] ✅ ({:?}, {:?})", $expected, $name);
    };

}

#[macro_export]
macro_rules! debug {
    // Match arm with format string parameter
    ($should_print:expr, $fmt:expr, $($arg:tt)*) => {
        if $should_print {
            let message = format!($fmt, $($arg)*);
            debug!(true, message);
        }
    };

    // Match arm with a condition and a single message
    ($should_print:ident, $message:expr) => {
        if $should_print {
            println!("[{}:{}] {:#?}", file!(), line!(), $message);
        }
    };

    (true, $message:expr) => {
        println!("[{}:{}] {}", file!(), line!(), $message);
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
        #[derive(PartialEq, Debug, Clone, Eq, Hash, Copy, PartialOrd)]
        pub enum $name {
            $($variant$(($opt))?),*
        }

        impl $name {
            pub fn from_char(c: char) -> $name {
                match c {
                    $(
                        $char => $name::$variant$(($opt::default()))?,
                    )*
                    _ => panic!("Invalid {} char: '{}'.", stringify!($name), c),
                }
            }

            pub fn to_char(&self) -> char {
                match self {
                    $(
                        $name::$variant$(($opt::default()))? => $char,
                    )*
                }
            }
        }
    };
}

/// A Vec<String> from a Vec<&str>
#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),* $(,)?) => (vec![$($x.to_string()),*]);
}

/// A String from a Vec<&str>
#[macro_export]
macro_rules! string {
    ($($x:expr),* $(,)?) => (vec![$($x.to_string()),*].join("\n"));
}

/// A Vec<Vec<Char>> from a Vec<&str>
#[macro_export]
macro_rules! grid_of_chars {
    ($($x:expr),* $(,)?) => (vec![$($x.chars().collect::<Vec<char>>()),*]);
}

#[macro_export]
macro_rules! clear_console {
    () => {
        print!("{}[2J", 27 as char);
    };
}

#[macro_export]
macro_rules! benchmark_functions {
    ($($func:expr),+) => {
            $(
                let start = Instant::now();
                let result = $func();
                let duration = start.elapsed();
                println!("{}() took {:?} and returned {}", stringify!($func), duration, result);
            )+
    };
}
