use crate::array::array::Array;
use crate::array::methods::multiply_row;
use crate::array::methods::multiply_add_row;
use crate::array::field_methods::LinearSystemResult;
use crate::array::float_eq::FloatEq;
use std::fmt;
use std::ops::{Add, Sub, Neg, Mul, Div};
use num::traits::{One, Zero};

pub enum LuResult<T> {
    Single(Array<T>),
    Infinite((Array<T>, Array<T>)),
}

pub struct LuFactorization<T> {
    l:Array<T>,
    u:Array<T>,
    pub size:(usize, usize),
}

impl<T: PartialEq> PartialEq for LuFactorization<T> {
    fn eq(&self, other:&Self) -> bool {
        self.size == other.size && self.l == other.l && self.u == other.u
    }
}

impl<T: Copy + Clone + PartialEq + FloatEq> FloatEq for LuFactorization<T> {    
    fn float_eq(&self, other:&Self) -> bool  {
        self.size == other.size && self.l.float_eq(&other.l) && self.u.float_eq(&other.u)
    }
}

impl<T: fmt::Display> fmt::Display for LuFactorization<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.l, self.u)
    }
}

impl<T: Copy + Clone + Zero + One + PartialEq
+ Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Mul<Output = T> + Div<Output = T> + FloatEq>
LuFactorization<T> {
    pub fn new(mut a:Array<T>) -> Result<Self, String> {
        let mut pivot = (0, 0);
        let mut l = Array::new_filled((a.size.1, a.size.1), T::zero());
        
        while pivot.0 < a.size.1 && pivot.1 < a.size.0 {
            if a[pivot].float_eq(&T::zero()) {
                return Err("There exists no LU factorization of this matrix".to_string())
            }
            let factor = T::one()/a[pivot];
            l[pivot] = a[pivot] * factor;
            for row in (pivot.0 + 1)..a.size.1 {
                if !T::is_zero(&a[(row, pivot.1)]) {
                    let u_factor = factor * -a[(row, pivot.1)];
                    l[(row, pivot.1)] = a[(row, pivot.1)] * factor;
                    multiply_add_row(&mut a, pivot.0, row, u_factor, pivot.1);
                }
            }
            pivot.0 += 1;
            pivot.1 += 1;
        }

        let size = a.size;
        Ok(
            LuFactorization {
                l,
                u:a,
                size,
            }
        )
    }

    fn solve_l(&self, y:Array<T>) -> Array<T> {
        let mut temp = Array::concat_0_axis(self.u.clone(), y);        
        let mut pivot = (temp.size.1 - 1, 0);
        while temp[pivot].float_eq(&T::zero()) {
            pivot.1 += 1;
            if pivot.1 >= temp.size.0 {
                if pivot.0 == 0 {
                    break;
                }
                pivot.0 -= 1;
                pivot.1 = 0;
            } 
        }
        loop {
            while pivot.1 > 0 && temp[(pivot.0, pivot.1 - 1)] != T::zero() {
                pivot.1 -= 1;
            }
            let factor = T::one() / temp[pivot];
            multiply_row(&mut temp, pivot.0, factor, pivot.1);
            for row in 0..pivot.0 {
                let factor = -temp[(row, pivot.1)];
                multiply_add_row(&mut temp, pivot.0, row, factor, pivot.1);
            }
            if pivot.0 == 0 {
                break;
            }
            pivot.0 -= 1;
        }
        temp
    }

    pub fn solve(&self, b:Array<T>) -> LuResult<T> {
        if b.size.1 != self.u.size.1 {
            panic!("The height of the A matrix and the b vector must be equal.");
        }
        let y = {
            let mut temp = Array::concat_0_axis(self.l.clone(), b);
            temp.echelon_form();
            Array::split_0_axis(temp, self.l.size.0).1
        };
        let res = self.solve_l(y);
        match Array::extract_solution_from_matrix(res, self.u.clone()) {
            LinearSystemResult::Single(res) => LuResult::Single(res),
            LinearSystemResult::Infinite(res) => LuResult::Infinite(res),
            LinearSystemResult::Inconsistent => panic!("Faulty implementation: Inconsistent system of equations."),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::array::factorizations::Array;
    use crate::array::float_eq::FloatEq;
    use crate::array::factorizations::LuFactorization;
    use crate::array::factorizations::LuResult;

    #[test]
    fn lu_factorization_possible() {
        let expected = LuFactorization{
            l:Array {
                content:vec![
                    vec![1.0, 0.0, 0.0],
                    vec![3.0, 1.0, 0.0],
                    vec![0.5, 0.16666666666666666, 1.0],
                ],
                size:(3, 3),
            },
            u:Array {
                content:vec![
                    vec![1.0, 3.0, 3.0],
                    vec![0.0, -3.0, 0.0],
                    vec![0.0, 0.0, 0.5],
                ],
                size:(3, 3),
            },
            size:(3, 3),
        };
        let actual = LuFactorization::new(Array {
            content:vec![
                vec![1.0, 3.0, 3.0],
                vec![3.0, 6.0, 9.0],
                vec![0.5, 1.0, 2.0],
            ],
            size:(3, 3),
        });
        match actual {
            Ok(arr) => assert!(expected.float_eq(&arr)),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn lu_factorization_impossible() {
        let actual = LuFactorization::new(Array {
            content:vec![
                vec![1.0, 3.0, 3.0],
                vec![3.0, 9.0, 9.0],
                vec![0.5, 1.0, 2.0],
            ],
            size:(3, 3),
        });
        match actual {
            Ok(arr) => panic!("Wrong result: {}", arr),
            Err(_) => {},
        }
    }

    #[test]
    fn solve() {
        let expected = Array {
            content:vec![vec![-16.0], vec![0.3333333333333333], vec![5.333333333333333]],
            size:(1, 3),
        };
        let actual = {
            let temp = LuFactorization{
                l:Array {
                    content:vec![
                        vec![1.0, 0.0, 0.0],
                        vec![3.0, 1.0, 0.0],
                        vec![0.5, 0.16666666666666666, 1.0],
                    ],
                    size:(3, 3),
                },
                u:Array {
                    content:vec![
                        vec![1.0, 3.0, 3.0],
                        vec![0.0, -3.0, 0.0],
                        vec![0.0, 0.0, 0.5],
                    ],
                    size:(3, 3),
                },
                size:(3, 3),
            };
            temp.solve(
                Array {
                    content:vec![vec![1.0], vec![2.0], vec![3.0]],
                    size:(1, 3),
                }
            )
        };
        match actual {
            LuResult::Single(s) => {
                println!("{}", s);
                assert!(expected.float_eq(&s))
            },
            LuResult::Infinite(s) => panic!("Wrong result: {}; {}", s.0, s.1),
        }
    }
}