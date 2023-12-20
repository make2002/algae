use std::env;

mod array;
use array::array::Array;
use crate::array::factorizations::LuFactorization;
use crate::array::factorizations::LuResult;
use crate::array::methods::LinearSystemResult;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    let mat_1 = vec![vec![1.0, 2.0, 3.0], vec![1.0, 3.0, 2.0], vec![3.0, 9.0, 6.0], vec![3.0, 9.0, 8.0]];
    let mat_2 = vec![vec![4.0], vec![6.0], vec![12.0], vec![12.0]];

    let mut mat_1 = Array::new_mat(mat_1);
    let mut mat_2 = Array::new_mat(mat_2);
    
    if let Ok(lu) = LuFactorization::new(mat_1) {
        match lu.solve(mat_2) {
            LuResult::Single(s) => {
                println!("{}", s);
            },
            LuResult::Infinite((fixed, free)) => {
                println!("{}; {}", fixed, free);
            },
        }
    } else {
        println!("Shit");
    }
}
