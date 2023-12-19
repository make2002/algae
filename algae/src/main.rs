use std::env;

mod array;
use array::array::Array;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    let mat_1 = vec![vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 2.0], vec![3.0, 2.0, 1.0]];
    let mat_2 = vec![vec![2.0, 2.0, 3.0], vec![4.0, 4.0, 6.0], vec![3.0, 0.0, 1.0]];
    let mat_3 = vec![vec![0, 1], vec![1, 0]];
    let mat_4 = vec![vec![2, 1], vec![1, 5]];

    let mut mat_1 = Array::new_mat(mat_1);
    let mut mat_2 = Array::new_mat(mat_2);
    let mut mat_3 = Array::new_mat(mat_3);
    let mut mat_4 = Array::new_mat(mat_4);
    
    println!("{}", mat_2.determinant());
}
