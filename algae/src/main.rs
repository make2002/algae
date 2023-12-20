use std::env;

mod array;
use array::array::Array;
use crate::array::factorizations::LuFactorization;
use crate::array::factorizations::LuResult;
use crate::array::methods::LinearSystemResult;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    let consumption = Array::new_mat(
        vec![
            vec![3.0, -2.0,], 
            vec![-5.0, 4.0,]
            ]
        );
    let demand = Array::new_vec(
        vec![6.0, 8.0]
    );

    let res = Array::cramers_rule(consumption.clone(), demand);
    println!("{}", res);
    println!("{}", consumption * res);
}
