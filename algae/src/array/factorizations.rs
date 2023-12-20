use crate::Array;
use crate::array::methods::multiply_add_row;
use std::fmt;
use std::ops::{Add, Sub, Neg, Mul, Div};
use num::traits::{One, Zero};

pub struct LuFactorization<T> {
    l:Array<T>,
    u:Array<T>,
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

        Ok(
            LuFactorization {
                l,
                u:a,
            }
        )
    }
}