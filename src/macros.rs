/*
Author: Ben Mezger (github.com/benmezger)
*/

/// Prints message, filename, and line number to stantard output
#[macro_export]
macro_rules! dbg {
    /* TODO: use eprintln instead of println */
    () => {
        crate::println!("[{}:{}]", file!(), line!());
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                crate::println!("[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(dbg!($val)),+,)
    };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($x:expr) => {
        dbg!($x)
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($( $args:expr ),*) => {};
}

/// Prints message and file information to standard output.
///
/// This requires `debug_assertions` to be enabled (compiled in development)
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! trace {
    ($($args: expr),*) => {
        crate::println!("TRACE: file: {}, line: {}", file!(), line!());
        $(
            crate::print!("{} ", stringify!($args));
        )*
        crate::println!();
    }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! trace {
    ($( $args:expr ),*) => {};
}

#[macro_export]
macro_rules! dbgf {
    (
        $fmt:expr, $val:expr $(,)?
    ) => {
        match $val {
            tmp => {
                crate::println!(
                    concat!("[{}:{}] ", $fmt),
                    file!(),
                    line!(),
                    format_args!("{} = {:?}", stringify!($val), tmp,),
                );
                tmp
            }
        }
    };
    (
        $val:expr $(,)?
    ) => {
        dbgf!("{}", $val)
    };
}
