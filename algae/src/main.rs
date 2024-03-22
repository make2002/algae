use std::env;

pub mod array;
pub mod ml;
mod signal_processing;
mod util;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
}
