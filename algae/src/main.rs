use std::env;

mod array;
use array::array::Array;


fn main() {
    env::set_var("RUST_BACKTRACE", "full");
}
