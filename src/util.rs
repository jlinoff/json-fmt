#![macro_use]

#[allow(unused_macros)]
macro_rules! info {
    ($($arg:expr), *) => {
        println!("INFO:{}:{}: {}", file!(), line!(), format! {$($arg), *});
    };
}

#[allow(unused_macros)]
macro_rules! infov {
    ($options:ident, $level: expr, $($arg:expr), *) => {
        if $options.verbose >= $level {
            println!("INFO:{}:{}: {}", file!(), line!(), format! {$($arg), *});}
    };
}

#[allow(unused_macros)]
macro_rules! warn {
    ($($arg:expr), *) => {
        eprintln!("\x1b[31mWARNING:{}:{}: {}\x1b[0m", file!(), line!(), format! {$($arg), *});
    };
}

#[allow(unused_macros)]
macro_rules! err {
    ($($arg:expr), *) => {
        eprintln!("\x1b[31mERROR:{}:{}: {}\x1b[0m", file!(), line!(), format! {$($arg), *});
        process::exit(1);
    };
}
