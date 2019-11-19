use std::env;
use json_fmt::run;

fn main() {
    // Make testing easier.
    let args: Vec<String> = env::args().collect();
    run(&args);
}
