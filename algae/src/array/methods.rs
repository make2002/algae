use crate::array::array::Array;
use std::ops::{Add, Sub, Neg, Mul, Div};
use num::traits::{One, Zero};

pub(in crate::array) fn multiply_row<T: Copy + Clone + Mul<Output = T>>
(a:&mut Array<T>, row:usize, factor:T, pivot_col:usize) {
    for col in pivot_col..a.size.0 {
        a[(row, col)] = a[(row, col)] * factor;
    }
}

pub(in crate::array) fn multiply_add_row<T: Copy + Clone + Add<Output = T> + Mul<Output = T>>
(a:&mut Array<T>, from_row:usize, to_row:usize, factor:T, pivot_col:usize) {
    for col in pivot_col..a.size.0 {
        a[(to_row, col)] = a[(to_row, col)] + a[(from_row, col)] * factor;
    }
}

impl<T: Copy + Clone + Zero + One + PartialEq
 + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Mul<Output = T> + Div<Output = T>> 
Array<T> {
    pub fn identity(size:usize) -> Self {
        let mut content = Vec::<Vec<T>>::with_capacity(size);
        for i in 0..size {
            let mut temp = Vec::<T>::with_capacity(size);
            for j in 0..size {
                if i == j {
                    temp.push(T::one());
                } else {
                    temp.push(T::zero());
                }
            }
            content.push(temp);
        }
        Array::new_mat(content)
    }

    pub fn elementary_swap(size:usize, swap:(usize, usize)) -> Self {
        let mut ret = Array::identity(size);
        ret[(swap.0, swap.0)] = T::zero();
        ret[(swap.1, swap.1)] = T::zero();
        ret[(swap.1, swap.0)] = T::one();
        ret[swap] = T::one();
        ret
    }

    pub fn elementary_multiply(size:usize, row:usize, factor:T) -> Self {
        let mut ret = Array::identity(size);
        ret[(row, row)] = factor;
        ret
    }

    pub fn elementary_add_into(size:usize, from_row:usize, to_row:usize, factor:T) -> Self {
        let mut ret = Array::identity(size);
        ret[(to_row, from_row)] = factor;
        ret
    }

    pub fn determinant(&self) -> T {
        let mut a = self.clone();
        let mut pivot = (0, 0);
        let mut det = T::one();
        while pivot.0 < a.size.1 && pivot.1 < a.size.0 {
            // Ensure pivot position != 0
            loop {
                let mut temp = Vec::<Vec<T>>::with_capacity(a.size.1);
                while pivot.0 < a.content.len() && T::is_zero(&a[pivot]) {
                    temp.push(a.content.swap_remove(pivot.0));
                }
                if pivot.0 >= a.content.len() {
                    return T::zero();
                } 
                temp = temp.clone().into_iter().rev().collect();
                match temp.pop() {
                    Some(t) => {
                        temp.insert(0, t);
                        // Swap pivot row with a.content.len()
                        det = -det;
                        a.content.append(&mut temp);
                    },
                    None => {},
                }
                if pivot.1 >= a.size.0 {
                    return T::zero();
                }
                if !T::is_zero(&a[pivot]) {
                    break;
                }
            }
            det = det * a[pivot];
            let factor = T::one() / a[pivot];
            multiply_row(&mut a, pivot.0, factor, pivot.1);
            for row in (pivot.0 + 1)..a.size.1 {
                if !T::is_zero(&a[(row, pivot.1)]) {
                    let factor = -a[(row, pivot.1)];
                    multiply_add_row(&mut a, pivot.0, row, factor, pivot.1);
                }
            }
            pivot.0 += 1;
            pivot.1 += 1;
        }
        det
    }
}

#[cfg(test)]
mod tests{
    use crate::array::methods::Array;
    use crate::array::methods::multiply_row;
    use crate::array::methods::multiply_add_row;
    use crate::array::float_eq::FloatEq;

    #[test]
    fn test_multiply_row() {
        let expected = Array {
            content:vec![vec![2, 4], vec![1, 0]],
            size:(2, 2),
        };
        let actual = {
            let mut temp = Array {
                content:vec![vec![1, 2], vec![1, 0]],
                size:(2, 2),
            };
            multiply_row(&mut temp, 0, 2, 0);
            temp
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multiply_add_row() {
        let expected = Array {
            content:vec![vec![1, 0], vec![0, 1]],
            size:(2, 2),
        };
        let actual = {
            let mut temp = Array {
                content:vec![vec![1, 0], vec![2, 1]],
                size:(2, 2),
            };
            multiply_add_row(&mut temp, 0, 1, -2, 0);
            temp
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn identity() {
        let expected = Array {
            content:vec![vec![1, 0], vec![0, 1]],
            size:(2, 2),
        };
        let actual = Array::identity(2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn elementary_swap() {
        let expected = Array {
            content:vec![vec![1, 0, 0], vec![0, 0, 1], vec![0, 1, 0]],
            size:(3, 3),
        };
        let actual = Array::elementary_swap(3, (1, 2));
        assert_eq!(expected, actual);
    }

    #[test]
    fn elementary_multiply() {
        let expected = Array {
            content:vec![vec![1, 0, 0], vec![0, 3, 0], vec![0, 0, 1]],
            size:(3, 3),
        };
        let actual = Array::elementary_multiply(3, 1, 3);
        assert_eq!(expected, actual);
    }

    #[test]
    fn elementary_add_into() {
        let expected = Array {
            content:vec![vec![1, 0, 0], vec![3, 1, 0], vec![0, 0, 1]],
            size:(3, 3),
        };
        let actual = Array::elementary_add_into(3, 0, 1, 3);
        assert_eq!(expected, actual);
    }

    #[test]
    fn elementary_multiply_equivalency() {
        let a = Array {
            content:vec![vec![1, 0, 0], vec![0, 3, 0], vec![0, 0, 1]],
            size:(3, 3),
        };
        let expected = {
            let mut temp = a.clone(); 
            multiply_row(&mut temp, 1, 3, 0);
            temp
        };
        let actual = Array::elementary_multiply(3, 1, 3) * a;
        assert_eq!(expected, actual);
    }

    #[test]
    fn elementary_add_into_equivalency() {
        let a = Array {
            content:vec![vec![1, 0, 0], vec![0, 3, 0], vec![0, 0, 1]],
            size:(3, 3),
        };
        let expected = {
            let mut temp = a.clone(); 
            multiply_add_row(&mut temp, 0, 1, 3, 0);
            temp
        };
        let actual = Array::elementary_add_into(3, 0, 1, 3) * a;
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
        let expected:f64 = 1.0;
        let actual = Array {
            content:vec![
                vec![ 3.0, 1.0, 0.0],
                vec![ 9.0, 3.0, 1.0],
                vec![19.0, 6.0, 2.0]
            ],
            size:(3, 3)
        }.determinant();
        assert!(expected.float_eq(&actual));
    }
}