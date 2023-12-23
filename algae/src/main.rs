use std::env;

mod array;
use array::array::Array;
use crate::array::methods::LinearSystemResult;


fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    let actual = {
        let a = Array::new_mat(
            vec![
                vec![1.0, 3.0, 3.0],
                vec![3.0, 6.0, 9.0],
                vec![0.5, 1.0, 2.0],
            ]
        );
        let b = Array::new_vec(vec![1.0, 2.0, 3.0]);
        Array::solve(a, b)
    };
    if let Ok(LinearSystemResult::Single(arr)) = actual {
        println!("{}", arr);
    }
}
