use std::env;

mod array;
mod ml;
mod signal_processing;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
}
