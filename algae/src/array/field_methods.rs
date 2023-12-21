use crate::array::array::Array;
use crate::array::methods::multiply_row;
use crate::array::methods::multiply_add_row;
use crate::array::methods::LinearSystemResult;
use crate::array::float_eq::FloatEq;
use std::ops::{Add, Sub, Neg, Mul, Div};
use num::traits::{One, Zero};

impl<T: Copy + Clone + FloatEq> FloatEq for Array<T> {
    fn zero_threshold() -> Array<T> {
        Array::new_mat(Vec::<Vec<T>>::new())
    }

    fn float_eq(&self, other:&Self) -> bool {
        if self.size != other.size {return false;}
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                if !self[(i, j)].float_eq(&other[(i, j)]) {return false;}
            }
        }
        true
    }
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
                temp = temp.clone().into_iter().rev().collect();
                match temp.pop() {
                    Some(t) => {
                        temp.insert(0, t);
    
                        self.content.append(&mut temp);
                    },
                    None => {},
                }
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
        while pivot.0 >= 0 && self[pivot].float_eq(&T::zero()) {
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
    }pub(in crate::array) fn extract_solution_from_matrix(res:Array<T>, a:Array<T>)
    -> Result<LinearSystemResult<T>, String> {
        let (mut mat, mut fixed) = Array::split_0_axis(res.clone(), a.size.0);
        // Might need a check if the outer split is even possible
        let mat = Array::split_1_axis(
            mat.clone(),
            a.size.0,
        ).0;
        let mut fixed = match Array::split_1_axis(
            fixed.clone(),
            a.size.0,
        ) {
            (i, j) if j.float_eq(&Array::new_filled(j.size, T::zero())) => {
                i
            },
            _ => return Err("No solution for this system of equations".to_string()),
        };
        if mat.float_eq(&Array::identity(mat.size.0)) {
            Ok(LinearSystemResult::Single(fixed))
        } else {
            // Find the last pivot position and determine, whether it is
            let mut pivot = (res.size.1 - 1, 0);
            while pivot.0 >= 0 && res[pivot].float_eq(&T::zero()) {
                pivot.1 += 1;
                if pivot.1 >= res.size.0 {
                    if pivot.0 == 0 {
                        break;
                    }
                    pivot.0 -= 1;
                    pivot.1 = 0;
                } 
            }
            if pivot.1 > mat.size.0 {
                return Err("No solution for this system of equations".to_string());
            }
            if a.size.0 < fixed.size.1 {
                let temp = Array::split_1_axis(fixed.clone(), a.size.0);
                if temp.1 != Array::new_filled(temp.1.size, T::zero()) {
                    return Err("No solution for this system of equations".to_string());
                }
                fixed = temp.0;
            } else {
                fixed.extend_to((fixed.size.0, a.size.0), T::zero());
            }
            let mut free_variables:Option<Array<T>> = None;
            let mut pivot_row = 0;
            for col_index in 0..mat.size.0 {
                if pivot_row >= mat.size.1 || mat[(pivot_row, col_index)].float_eq(&T::zero()) {
                    let mut col = -mat.get_col(col_index);
                    col.extend_to((1, a.size.0), T::one());
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
            match free_variables {
                Some(arr) => {
                    // here fixed + any linear combination of the column vectors in arr will yield a result.
                    Ok(LinearSystemResult::Infinite((fixed, arr)))
                },
                None => {
                    panic!("Faulty implementation");
                }
            }
        }  
    }

    /*
    pub(in crate::array) fn extract_solution_from_matrix(res:(Array<T>, Array<T>), a:Array<T>)
    -> Result<LinearSystemResult<T>, String> {
        if res.0 == Array::identity(res.0.size.0) {
            Ok(LinearSystemResult::Single(res.1))
        } else {
            let mut fixed = res.1.clone();
            // Find the last pivot position and determine, whether it is
            let joint = Array::concat_0_axis(res.0.clone(), res.1.clone());
            let mut pivot = (joint.size.1 - 1, 0);
            while pivot.0 >= 0 && joint[pivot] == T::zero() {
                pivot.1 += 1;
                if pivot.1 >= joint.size.0 {
                    if pivot.0 == 0 {
                        break;
                    }
                    pivot.0 -= 1;
                    pivot.1 = 0;
                } 
            }
            if pivot.1 > res.0.size.0 {
                return Err("No solution for this system of equations".to_string());
            }
            if a.size.0 < fixed.size.1 {
                let temp = Array::split_1_axis(fixed.clone(), a.size.0);
                if temp.1 != Array::new_filled(temp.1.size, T::zero()) {
                    return Err("No solution for this system of equations".to_string());
                }
                fixed = temp.0;
            } else {
                fixed.extend_to((fixed.size.0, a.size.0), T::zero());
            }
            let mut free_variables:Option<Array<T>> = None;
            let mut pivot_row = 0;
            for col_index in 0..res.0.size.0 {
                if pivot_row >= res.0.size.1 || res.0[(pivot_row, col_index)] == T::zero() {
                    let mut col = -res.0.get_col(col_index);
                    col.extend_to((1, a.size.0), T::one());
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
            match free_variables {
                Some(arr) => {
                    // here fixed + any linear combination of the column vectors in arr will yield a result.
                    Ok(LinearSystemResult::Infinite((fixed, arr)))
                },
                None => {
                    panic!("Faulty implementation");
                }
            }
        }        
    } */

    pub fn solve(a:Array<T>, b:Array<T>) -> Result<LinearSystemResult<T>, String> {
        if a.size.1 != b.size.1 {
            panic!("The height of the A matrix and the b vector must be equal.");
        } 
        let split_col = a.size.0;
        let mut m = Array::concat_0_axis(a.clone(), b);
        m.reduced_echelon_form();
        Self::extract_solution_from_matrix(m, a)
    }

    pub fn inv(&self) -> Result<Array<T>, String> {
        match Self::solve(self.clone(), Self::identity(self.size.0)) {
            Ok(LinearSystemResult::Single(r)) => {
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
        while pivot.0 >= 0 && a[pivot].float_eq(&T::zero()) {
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
    -> Result<LinearSystemResult<T>, String> {
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
            Ok(LinearSystemResult::Single(res)) => res,
            Ok(LinearSystemResult::Infinite(res)) => res.1,
            Err(e) => panic!("Faulty implementation: {}", e),
        }
    }
}


#[cfg(test)]
mod tests{
    use crate::Array;
    use crate::array::methods::multiply_row;
    use crate::array::methods::multiply_add_row;

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
    fn determinant_0() {
        let expected = 0.0;
        let actual = Array {
            content:vec![
                vec![1.0, 2.0, 3.0],
                vec![1.0, 1.0, 3.0],
                vec![3.0, 3.0, 9.0]
            ],
            size:(3, 3)
        }.determinant();
        assert_eq!(expected, actual);
    }

    #[test]
    fn determinant() {
        let expected = 1.0;
        let actual = Array {
            content:vec![
                vec![ 3.0, 1.0, 0.0],
                vec![ 9.0, 3.0, 1.0],
                vec![19.0, 6.0, 2.0]
            ],
            size:(3, 3)
        }.determinant();
        // Arbitrary
        assert!(f64::abs(expected - actual) < 0.000001);
    }
}