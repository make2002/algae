
use crate::Array;
use std::ops::{Add, Sub, Neg, Mul, Div};
use num::traits::{One, Zero};

fn multiply_row<T: Copy + Clone + Mul<Output = T>>
(a:&mut Array<T>, row:usize, factor:T) {
    for col in 0..a.size.0 {
        a[(row, col)] = a[(row, col)] * factor;
    }
}

fn multiply_add_row<T: Copy + Clone + Add<Output = T> + Mul<Output = T>>
(a:&mut Array<T>, from_row:usize, to_row:usize, factor:T) {
    for col in 0..a.size.0 {
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

    pub fn echelon_form(&mut self) -> Self {
        let mut pivot = (0, 0);
        let mut elementary_mat = Array::identity(self.size.1);
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

                        elementary_mat = Array::elementary_swap(
                            elementary_mat.size.0, 
                            (self.content.len(), pivot.0)
                        ) * elementary_mat;
    
                        self.content.append(&mut temp);
                    },
                    None => {},
                }
                if pivot.1 >= self.size.0 {
                    return elementary_mat;
                }
                if !T::is_zero(&self[pivot]) {
                    break;
                }
            }
            let factor = T::one()/self[pivot];
            multiply_row(self, pivot.0, factor);
            elementary_mat = Array::elementary_multiply(
                elementary_mat.size.0,
                pivot.0,
                factor
            ) * elementary_mat;
            for row in (pivot.0 + 1)..self.size.1 {
                if !T::is_zero(&self[(row, pivot.1)]) {
                    let factor = -self[(row, pivot.1)];
                    multiply_add_row(self, pivot.0, row, factor);
                    elementary_mat = Array::elementary_add_into(
                        elementary_mat.size.0,
                        pivot.0,
                        row,
                        factor
                    ) * elementary_mat;
                }
            }
            pivot.0 += 1;
            pivot.1 += 1;
        }
        elementary_mat
    }

    pub fn reduced_echelon_form(&mut self) -> Self {
        let mut elementary_mat = self.clone().echelon_form();
        *self = elementary_mat.clone() * self.clone();
        
        let mut pivot = (self.size.1 - 1, self.size.0 - 1);
        while pivot.0 > 0 {
            while pivot.1 > 0 && self[(pivot.0, pivot.1 - 1)] != T::zero() {
                pivot.1 -= 1;
            }
            for row in 0..pivot.0 {
                let factor = -self[(row, pivot.1)];
                multiply_add_row(self, pivot.0, row, factor);
                elementary_mat = Array::elementary_add_into(
                    elementary_mat.size.0,
                    pivot.0,
                    row,
                    factor
                ) * elementary_mat;
            }
            pivot.0 -= 1;
        }
        elementary_mat
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
            let factor = T::one() / a[pivot];
            multiply_row(&mut a, pivot.0, factor);
            det = det * factor;
            for row in (pivot.0 + 1)..a.size.1 {
                if !T::is_zero(&a[(row, pivot.1)]) {
                    let factor = -a[(row, pivot.1)];
                    multiply_add_row(&mut a, pivot.0, row, factor);
                }
            }
            pivot.0 += 1;
            pivot.1 += 1;
        }
        det
    }
}