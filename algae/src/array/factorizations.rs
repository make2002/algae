use crate::Array;
use crate::array::methods::multiply_add_row;
use crate::array::methods::LinearSystemResult;
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

impl<T: fmt::Display> fmt::Display for LuFactorization<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.l, self.u)
    }
}

impl<T: Copy + Clone + Zero + One + PartialEq
+ Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Mul<Output = T> + Div<Output = T> + std::fmt::Display>
LuFactorization<T> {
    pub fn new(mut a:Array<T>) -> Result<Self, String> {
        let mut pivot = (0, 0);
        let mut l = Array::new_filled((a.size.1, a.size.1), T::zero());
        
        let mut pivot = (0, 0);
        while pivot.0 < a.size.1 && pivot.1 < a.size.0 {
            if a[pivot] == T::zero() {
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

    fn solve_l(&self, y:Array<T>) -> (Array<T>, Array<T>) {
        let mut temp = Array::concat_0_axis(self.u.clone(), y);        
        let mut pivot = (temp.size.1 - 1, 0);
        while pivot.0 >= 0 && temp[pivot] == T::zero() {
            pivot.1 += 1;
            if pivot.1 >= temp.size.0 {
                if pivot.0 == 0 {
                    break;
                }
                pivot.0 -= 1;
                pivot.1 = 0;
            } 
        }
        while pivot.0 > 0 {
            while pivot.1 > 0 && temp[(pivot.0, pivot.1 - 1)] != T::zero() {
                pivot.1 -= 1;
            }
            temp[pivot] = temp[pivot] * T::one() / temp[pivot];
            for row in 0..pivot.0 {
                let factor = -temp[(row, pivot.1)];
                multiply_add_row(&mut temp, pivot.0, row, factor, pivot.1);
            }
            pivot.0 -= 1;
        }
        Array::split_0_axis(temp, self.u.size.0)
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
            Ok(LinearSystemResult::Single(res)) => LuResult::Single(res),
            Ok(LinearSystemResult::Infinite(res)) => LuResult::Infinite(res),
            Err(e) => panic!("Faulty implementation yielded: {}", e),
        }
    }
}