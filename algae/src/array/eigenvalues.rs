use crate::array::array::Array;
use crate::array::factorizations::LuFactorization;
use crate::array::factorizations::LuResult;
use crate::array::float_eq::FloatEq;
use std::ops::{Add, Sub, Neg, Mul, Div};
use num::traits::{One, Zero};

impl<T: Copy + Clone + Zero + One + PartialEq + PartialOrd
 + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Mul<Output = T> + Div<Output = T> + FloatEq + std::fmt::Display> 
Array<T> {
    pub fn power_method(array:&Array<T>, x_zero:Array<T>, iteration_cap:usize) 
    -> Result<(T, Array<T>), (String, T, Array<T>)> {
        let mut x_k_minus_one = x_zero;
        let mut mu_k_minus_one = T::zero();
        for _ in 0..iteration_cap {
            let mut x_k = array.clone() * x_k_minus_one.clone();
            let mu_k = {
                let mut max = x_k[(0, 0)];
                for i in 1..x_k.size.1 {
                    if x_k[(i, 0)] > max {
                        max = x_k[(i, 0)];
                    }
                }
                max
            };
            x_k = x_k.clone() * (T::one() / mu_k);
            if mu_k.float_eq(&mu_k_minus_one) && x_k.float_eq(&x_k_minus_one) {
                return Ok((mu_k, x_k))
            }
            mu_k_minus_one = mu_k;
            x_k_minus_one = x_k;
        }
        Err((
            format!("Power method exceeded iteration_cap: {}", iteration_cap),
            mu_k_minus_one, 
            x_k_minus_one,
        ))
    }

    pub fn inverse_power_method(array:&Array<T>, x_zero:Array<T>, alpha:T, iteration_cap:usize) 
    -> Result<(T, Array<T>), (String, T, Array<T>)> {
        let mut v_k_minus_one = alpha;
        let mut x_k_minus_one = x_zero.clone();
        let equation_system = match LuFactorization::new(
            array.clone() - (Array::identity(array.size.0) * alpha)
        ) {
            Ok(e_s) => e_s,
            Err(_) => return Err(("Non valid system of equations for this choice of alpha".to_string(), alpha, x_zero)),
        };
        for _ in 0..iteration_cap {
            let y_k = match equation_system.solve(x_k_minus_one.clone()) {
                LuResult::Single(r) => r,
                LuResult::Infinite((r, _)) => r,
            };
            let mu_k = {
                let mut max = y_k[(0, 0)];
                for i in 1..y_k.size.1 {
                    if y_k[(i, 0)] > max {
                        max = y_k[(i, 0)];
                    }
                }
                max
            };
            let v_k = (alpha + T::one()) / mu_k;
            let x_k = y_k * (T::one() / mu_k);
            if v_k.float_eq(&v_k_minus_one) && x_k.float_eq(&x_k_minus_one) {
                return Ok((v_k, x_k))
            }
            v_k_minus_one = v_k;
            x_k_minus_one = x_k;
        }
        Err((
            format!("Power method exceeded iteration_cap: {}", iteration_cap),
            v_k_minus_one,
            x_k_minus_one, 
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::array::eigenvalues::Array;
    use crate::array::float_eq::FloatEq;

    #[test]
    fn power_method() {
        let array = Array {
            content:vec![
                vec![1.0, 2.0],
                vec![5.0, 4.0],
            ],
            size:(2, 2),
        };
        let e_0 = Array {
            content:vec![
                vec![1.0],
                vec![0.0],
            ],
            size:(1, 2),
        };
        let iteration_cap = 10000;
        match Array::power_method(&array, e_0, iteration_cap) {
            Ok((lambda, x)) => {
                assert!((array * x.clone()).float_eq(&(x * lambda)))
            },
            Err((e, _, _)) => {
                panic!("{}", e)
            },
        }
    }

    #[test]
    fn inverse_power_method() {
        let array = Array {
            content:vec![
                vec![1.0, 2.0],
                vec![0.0, 4.0],
            ],
            size:(2, 2),
        };
        let e_0 = Array {
            content:vec![
                vec![1.0],
                vec![0.0],
            ],
            size:(1, 2),
        };
        let alpha = 0.0;
        let iteration_cap = 10000;
        match Array::inverse_power_method(&array, e_0, alpha, iteration_cap) {
            Ok((lambda, x)) => {
                assert!((array * x.clone()).float_eq(&(x * lambda)))
            },
            Err((e, _, _)) => {
                panic!("{}", e)
            },
        }
    }
}
