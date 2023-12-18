use numerical_methods_playground::Array;

fn main() {
    let mat_1 = vec![vec![1, 2, 3], vec![1, 2, 2], vec![3, 2, 1]];
    let mat_2 = vec![vec![2, 2, 3], vec![1, 3, 2], vec![3, 0, 1]];
    let mat_3 = vec![vec![0, 1], vec![1, 0]];
    let mat_4 = vec![vec![2, 1], vec![1, 5]];

    let mat_1 = Array::new_mat(mat_1);
    let mat_2 = Array::new_mat(mat_2);
    let mat_3 = Array::new_mat(mat_3);
    let mat_4 = Array::new_mat(mat_4);

    println!("{}", mat_3 * mat_4);
}
