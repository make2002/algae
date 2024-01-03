use crate::array::array::Array;
use crate::array::methods::multiply_row;
use crate::array::methods::multiply_add_row;
use crate::array::float_eq::FloatEq;
use std::ops::{Add, Sub, Neg, Mul, Div};
use num::traits::{One, Zero};

impl<T: Copy + Clone + FloatEq> FloatEq for Array<T> {
    fn float_eq(&self, other:&Self) -> bool {
        if self.size != other.size {return false;}
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                if !self[(j, i)].float_eq(&other[(j, i)]) {return false;}
            }
        }
        true
    }
}

pub enum LinearSystemResult<T> {
    Single(Array<T>),
    Infinite((Array<T>, Array<T>)),
    Inconsistent,
}

impl<T: Copy + Clone + Zero + One + PartialEq
 + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Mul<Output = T> + Div<Output = T> + FloatEq> 
Array<T> {
    pub fn echelon_form(&mut self) {
        let mut pivot = (0, 0);
        while pivot.0 < self.size.1 && pivot.1 < self.size.0 {
            // Ensure pivot position != 0
            loop {
                let mut temp = Vec::<Vec<T>>::with_capacity(self.size.1);
                while pivot.0 < self.content.len() && T::is_zero(&self[pivot]) {
                    temp.push(self.content.swap_remove(pivot.0));
                }
                if pivot.0 >= self.content.len() {
                    pivot.1 += 1;
                } 
                self.content.append(&mut temp);
                // This way the elementary matrix only performs a single swap
                // temp = temp.clone().into_iter().rev().collect();
                // match temp.pop() {
                //     Some(t) => {
                //         temp.insert(0, t);
                //         self.content.append(&mut temp);
                //     },
                //     None => {},
                // }
                if pivot.1 >= self.size.0 {
                    return;
                }
                if !T::is_zero(&self[pivot]) {
                    break;
                }
            }
            let factor = T::one()/self[pivot];
            multiply_row(self, pivot.0, factor, pivot.1);
            self[pivot] = T::one();
            for row in (pivot.0 + 1)..self.size.1 {
                if !T::is_zero(&self[(row, pivot.1)]) {
                    let factor = -self[(row, pivot.1)];
                    multiply_add_row(self, pivot.0, row, factor, pivot.1);
                }
            }
            pivot.0 += 1;
            pivot.1 += 1;
        }
    }

    pub(in crate::array) fn echelon_form_to_reduced_echelon_form(&mut self) {
        let mut pivot = (self.size.1 - 1, 0);
        while self[pivot].float_eq(&T::zero()) {
            pivot.1 += 1;
            if pivot.1 >= self.size.0 {
                if pivot.0 == 0 {
                    break;
                }
                pivot.0 -= 1;
                pivot.1 = 0;
            } 
        }
        while pivot.0 > 0 {
            while pivot.1 > 0 && self[(pivot.0, pivot.1 - 1)] != T::zero() {
                pivot.1 -= 1;
            }
            for row in 0..pivot.0 {
                let factor = -self[(row, pivot.1)];
                multiply_add_row(self, pivot.0, row, factor, pivot.1);
            }
            pivot.0 -= 1;
        }
    }

    pub fn reduced_echelon_form(&mut self) {
        self.echelon_form();
        self.echelon_form_to_reduced_echelon_form();
    }
    
    pub(in crate::array) fn extract_solution_from_matrix(res:Array<T>, mat:Array<T>)
    -> LinearSystemResult<T> {
        let (a, b) = Array::split_0_axis(res.clone(), mat.size.0);
        // Might need a check if the outer split is even possible
        let a = {
            let mut temp = Array::split_1_axis(
                a.clone(),
                a.size.0,
            ).0;
            while temp.get_row(temp.size.1 - 1) == Array::new_filled((temp.size.0, 1), T::zero()) {
                temp = Array::split_1_axis(
                    temp.clone(),
                    temp.size.1 - 1,
                ).0;
            }
            temp
        };
        let mut b = match Array::split_1_axis(
            b.clone(),
            a.size.0,
        ) {
            (i, j) if j.float_eq(&Array::new_filled(j.size, T::zero())) => {
                i
            },
            _ => return LinearSystemResult::Inconsistent,
        };
        while b.size.1 > 0 && b.get_row(b.size.1 - 1) == Array::new_filled((b.size.0, 1), T::zero()) {
            b = Array::split_1_axis(
                b.clone(),
                b.size.1 - 1,
            ).0;
        }
        if a.float_eq(&Array::identity(a.size.0)) {
            let mut r = b.clone();
            r.extend_to((b.size.0, mat.size.1), T::zero());
            LinearSystemResult::Single(r)
        } else if b.size.1 > a.size.1 {
            LinearSystemResult::Inconsistent
        } else {
            let mut pivot = (res.size.1 - 1, 0);
            while res[pivot].float_eq(&T::zero()) {
                pivot.1 += 1;
                if pivot.1 >= res.size.0 {
                    if pivot.0 == 0 {
                        break;
                    }
                    pivot.0 -= 1;
                    pivot.1 = 0;
                } 
            }
            let mut free_variables:Option<Array<T>> = None;
            let mut pivot_row = 0;
            for col_index in 0..a.size.0 {
                if pivot_row >= a.size.1 || a[(pivot_row, col_index)].float_eq(&T::zero()) {
                    let mut col = -a.get_col(col_index);
                    col.extend_to((1, mat.size.0), T::one());
                    match free_variables {
                        Some(arr) => {
                            free_variables = Some(Array::concat_0_axis(arr, col));
                        },
                        None => free_variables = Some(col),
                    }
                } else {
                    pivot_row += 1;
                }
            }
            b.extend_to((b.size.0, mat.size.1), T::zero());
            match free_variables {
                Some(arr) => {
                    // here fixed + any linear combination of the column vectors in arr will yield a result.
                    LinearSystemResult::Infinite((b, arr))
                },
                None => {
                    panic!("Faulty implementation");
                }
            }
        }
    }

    pub fn solve(a:Array<T>, b:Array<T>) -> LinearSystemResult<T> {
        if a.size.1 != b.size.1 {
            panic!("The height of the A matrix and the b vector must be equal.");
        } 
        let mut m = Array::concat_0_axis(a.clone(), b);
        m.reduced_echelon_form();
        Self::extract_solution_from_matrix(m, a)
    }

    pub fn inv(&self) -> Result<Array<T>, String> {
        match Self::solve(self.clone(), Self::identity(self.size.0)) {
            LinearSystemResult::Single(r) => {
                Ok(r)
            },
            _ => {
                Err("Non-invertible matrix.".to_string())
            },
        }
    }

    pub fn rank(&self) -> usize {
        let mut a = self.clone();
        a.echelon_form();
        let mut pivot = (a.size.1 - 1, 0);
        while a[pivot].float_eq(&T::zero()) {
            pivot.1 += 1;
            if pivot.1 >= a.size.0 {
                if pivot.0 == 0 {
                    break;
                }
                if pivot.0 == 0 {
                    return 0;
                }
                pivot.0 -= 1;
                pivot.1 = 0;
            } 
        }
        pivot.0 + 1
    }

    pub fn leontief_input_output_model(consumption:Array<T>, demand:Array<T>) 
    -> LinearSystemResult<T> {
        if consumption.size.0 != consumption.size.1 {
            panic!("A consumption matrix must be square.");
        }
        if demand.size.0 != 1 {
            panic!("Demand is a vector");
        }
        if demand.size.1 != consumption.size.1 {
            panic!("The demand vector must be as long as the consumption matrix is high.");
        }
        let a = Array::identity(consumption.size.0) - consumption;
        Array::solve(a, demand) 
    }

    fn replace_col(mut a:Array<T>, col_index:usize, col:Array<T>) -> Array<T> {
        for row_index in 0..a.size.1 {
            a[(row_index, col_index)] = col[(row_index, 0)];
        }
        a
    }

    pub fn cramers_rule(a:Array<T>, b:Array<T>) -> Array<T> {
        if a.size.1 != b.size.1 {
            panic!("a must be as tall as b");
        }
        if b.size.0 != 1 {
            panic!("b must be a vector");
        }
        let mut content = Vec::<Vec<T>>::with_capacity(b.size.1);
        let det_a = a.determinant();
        if det_a.float_eq(&T::zero()) {
            panic!("Cramers rule doesn't apply, when det(a) == 0");
        }
        for i in 0..b.size.1 {
            content.push(vec![
                Self::replace_col(a.clone(), i, b.clone()).determinant() / det_a
            ]);
        }
        Array {
            content,
            size:b.size,
        }
    }

    pub fn null_space(&self) -> Array<T> {
        match Array::solve(self.clone(), Array::new_filled((1, self.size.1), T::zero())) {
            LinearSystemResult::Single(res) => res,
            LinearSystemResult::Infinite(res) => res.1,
            LinearSystemResult::Inconsistent => panic!("Faulty implementation: Incosistent system of equations."),
        }
    }

    pub fn get_linear_independent(mut array:Self) -> Array<T> {
        let echelon_form = {
            let mut temp = array.clone();
            temp.echelon_form();
            temp
        };
        let mut pivot = (0, 0);
        let mut indicies = Vec::<usize>::with_capacity(echelon_form.size.0);
        while pivot.1 < echelon_form.size.0 {
            while   (pivot.0 >= echelon_form.size.1 && pivot.1 < echelon_form.size.0) ||
                    pivot.0 < echelon_form.size.1 &&
                    pivot.1 < echelon_form.size.0 &&
                    echelon_form[pivot].float_eq(&T::zero())
            {
                indicies.push(pivot.1);
                pivot.1 += 1;
            }
            pivot = (pivot.0 + 1, pivot.1 + 1);
        }
        while let Some(i) = indicies.pop() {
            if i + 1 < array.size.0 {
                let a = Array::split_0_axis(array.clone(), i).0;
                let b = Array::split_0_axis(array.clone(), i + 1).1;
                array = Array::concat_0_axis(a, b);
            } else {
                array = Array::split_0_axis(array.clone(), i).0;
            }
        }
        array
    }
    
    pub fn change_basis(basis_a:&Array<T>, vec_a:&Array<T>, basis_b:&Array<T>) -> Array<T> {
        let basis_b_inv = match basis_b.inv() {
            Ok(i) => i,
            Err(e) => panic!("Error: basis_b not a basis: {}", e),
        };
        basis_b_inv * basis_a.clone() * vec_a.clone()
    }
}


#[cfg(test)]
mod tests{
    use crate::array::field_methods::Array;
    use crate::array::float_eq::FloatEq;
    use crate::array::field_methods::LinearSystemResult;

    #[test]
    fn echelon_form() {
        let expected = Array {
            content:vec![vec![1.0, 2.0, 3.0], vec![0.0, 1.0, 6.0], vec![0.0, 0.0, 1.0]],
            size:(3, 3),
        };
        let actual = {
            let mut temp = Array {
                content:vec![
                    vec![1.0, 2.0, 3.0],
                    vec![1.0, 3.0, 9.0],
                    vec![2.0, 5.0, 11.0]
                ],
                size:(3, 3)
            };
            temp.echelon_form();
            temp
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn echelon_form_to_reduced_echelon_form() {
        let expected = Array {
            content:vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]],
            size:(3, 3),
        };
        let actual = {
            let mut temp = Array {
                content:vec![
                    vec![1.0, 2.0, 3.0],
                    vec![0.0, 1.0, 9.0],
                    vec![0.0, 0.0, 1.0]
                ],
                size:(3, 3)
            };
            temp.echelon_form_to_reduced_echelon_form();
            temp
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn reduced_echelon_form() {
        let expected = Array {
            content:vec![vec![1.0, 0.0, 3.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 0.0]],
            size:(3, 3),
        };
        let actual = {
            let mut temp = Array {
                content:vec![
                    vec![1.0, 2.0, 3.0],
                    vec![1.0, 1.0, 3.0],
                    vec![3.0, 3.0, 9.0]
                ],
                size:(3, 3)
            };
            temp.reduced_echelon_form();
            temp
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn solve_single() {
        let expected = Array {
            content:vec![vec![-16.0], vec![0.3333333333333333], vec![5.333333333333333]],
            size:(1, 3),
        };
        let actual = {
            let a = Array {
                content:vec![
                    vec![1.0, 3.0, 3.0],
                    vec![3.0, 6.0, 9.0],
                    vec![0.5, 1.0, 2.0],
                ],
                size:(3, 3),
            };
            let b = Array {
                content:vec![vec![1.0], vec![2.0], vec![3.0]],
                size:(1, 3),
            };
            Array::solve(a, b)
        };
        match actual {
            LinearSystemResult::Single(a) => {
                assert!(expected.float_eq(&a));
            }
            _ => {
                panic!("Wrong result");
            }
        }
    }

    // For reference
    // 1 0 1 | 1
    // 0 1 1 | 1
    // 0 0 0 | 0
    
    //  3 0 3 | 3
    // -1 1 0 | 0
    //  2 3 5 | 5
    #[test]
    fn solve_infinite() {
        let expected = (Array {
            content:vec![vec![1.0], vec![1.0], vec![0.0]],
            size:(1, 3),
        }, Array {
            content:vec![vec![-1.0], vec![-1.0], vec![1.0]],
            size:(1, 3),
        });
        let actual = {
            let a = Array {
                content:vec![
                    vec![3.0, 0.0, 3.0],
                    vec![-1.0, 1.0, 0.0],
                    vec![2.0, 3.0, 5.0],
                ],
                size:(3, 3),
            };
            let b = Array {
                content:vec![vec![3.0], vec![0.0], vec![5.0]],
                size:(1, 3),
            };
            Array::solve(a, b)
        };
        match actual {
            LinearSystemResult::Infinite(a) => {
                assert!(expected.0.float_eq(&a.0));
                assert!(expected.1.float_eq(&a.1));
            },
            LinearSystemResult::Single(s) => {
                panic!("Wrong result: {}", s);
            },
            LinearSystemResult::Inconsistent => {
                panic!("Wrong result: Linear system inconsistent.");
            },
        }
    }

    // 1 0 1 | 1
    // 0 1 0 | 1
    // 0 0 0 | 1

    // 1 3 1 | 4
    // 2 1 2 | 3
    // 4 0 4 | 5

    #[test]
    fn solve_unsolvable() {
        let actual = {
            let a = Array {
                content:vec![
                    vec![1.0, 3.0, 1.0],
                    vec![1.0, 2.0, 1.0],
                    vec![4.0, 0.0, 4.0],
                ],
                size:(3, 3),
            };
            let b = Array {
                content:vec![vec![4.0], vec![3.0], vec![5.0]],
                size:(1, 3),
            };
            Array::solve(a, b)
        };
        match actual {
            LinearSystemResult::Infinite(a) => {
                panic!("Wrong result: {}; {}", a.0, a.1);
            },
            LinearSystemResult::Single(s) => {
                panic!("Wrong result: {}", s);
            },
            _ => {}
        }
    }

    #[test]
    fn inverse() {
        let matrix = Array {
            content:vec![
                vec![1.0, 3.0, 3.0],
                vec![3.0, 6.0, 9.0],
                vec![0.5, 1.0, 2.0],
            ],
            size:(3, 3),
        };
        let inverse = match matrix.inv() {
            Ok(i) => i,
            Err(e) => panic!("Error: {}", e),
        };
        assert!(Array::identity(matrix.size.0).float_eq(&(inverse.clone() * matrix.clone())));
        assert!(Array::identity(matrix.size.0).float_eq(&(matrix.clone() * inverse.clone())));
    }

    #[test]
    fn rank() {
        let expected = 2;
        let actual = {
            let temp = Array {
                content:vec![
                    vec![3.0, 0.0, 3.0],
                    vec![-1.0, 1.0, 0.0],
                    vec![2.0, 3.0, 5.0],
                ],
                size:(3, 3),
            };
            temp.rank()
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn leontief_input_output_model_test() {
        let expected = Array {
            content:vec![vec![-16.0], vec![0.3333333333333333], vec![5.333333333333333]],
            size:(1, 3),
        };
        match {
            let consumption = Array {
                content:vec![
                    vec![0.0, -3.0, -3.0],
                    vec![-3.0, -5.0, -9.0],
                    vec![-0.5, -1.0, -1.0],
                ],
                size:(3, 3),
            };
            let demand = Array {
                content:vec![vec![1.0], vec![2.0], vec![3.0]],
                size:(1, 3),
            };
            Array::leontief_input_output_model(consumption, demand)
        } {
            LinearSystemResult::Single(actual) => assert!(expected.float_eq(&actual)),
            LinearSystemResult::Infinite(actual) => panic!("Wrong result: {}, {}", actual.0, actual.1),
            LinearSystemResult::Inconsistent => panic!("Error: Inconsistent system of equations."),
        }
    }

    #[test]
    fn cramers_rule_test() {
        let expected = Array {
            content:vec![vec![-16.0], vec![0.3333333333333333], vec![5.333333333333333]],
            size:(1, 3),
        };
        let actual = {
            let a = Array {
                content:vec![
                    vec![1.0, 3.0, 3.0],
                    vec![3.0, 6.0, 9.0],
                    vec![0.5, 1.0, 2.0],
                ],
                size:(3, 3),
            };
            let b = Array {
                content:vec![vec![1.0], vec![2.0], vec![3.0]],
                size:(1, 3),
            };
            Array::cramers_rule(a, b)
        };
        assert!(expected.float_eq(&actual));
    }

    #[test]
    fn null_space() {
        let expected = Array{
            content:vec![vec![-1.0], vec![-1.0], vec![1.0]],
            size:(1, 3),
        };
        let actual = {
            Array {
                content:vec![
                    vec![3.0, 0.0, 3.0],
                    vec![-1.0, 1.0, 0.0],
                    vec![2.0, 3.0, 5.0],
                ],
                size:(3, 3),
            }.null_space()
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_linear_independent_test() {
        let expected = Array {
            content:vec![
                vec![1.0, 3.0],
                vec![2.0, 3.0],
                vec![2.0, 1.0],
            ],
            size:(2, 3),
        };
        let actual = Array::get_linear_independent(Array {
            content:vec![
                vec![1.0, 2.0, 3.0, 6.0],
                vec![2.0, 4.0, 3.0, 6.0],
                vec![2.0, 4.0, 1.0, 2.0],
            ],
            size:(4, 3),
        });
        assert_eq!(expected, actual);
    }

    #[test]
    fn change_basis_test() {
        let basis_a = Array {
            content:vec![
                vec![3.0, 4.0, 0.0],
                vec![0.0, 4.0, 4.0],
                vec![1.0, 0.0, 4.0],
            ],
            size:(3, 3),
        };
        let basis_b = Array {
            content:vec![
                vec![1.0, 1.0, 1.0],
                vec![1.0, 2.0, 3.0],
                vec![1.0, 4.0, 9.0],
            ],
            size:(3, 3),
        };
        let vec_a = Array {
            content:vec![
                vec![1.0],
                vec![0.0],
                vec![0.0],
            ],
            size:(1, 3),
        };
        let vec_b = Array::change_basis(&basis_a, &vec_a, &basis_b);
        let vec_a_back = Array::change_basis(&basis_b, &vec_b, &basis_a);
        assert!(vec_a.float_eq(&vec_a_back));
    }
}