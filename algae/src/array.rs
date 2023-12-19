pub mod array {
    use std::ops::{Index, IndexMut, Add, Sub, Neg, Mul, Div};
    use num::traits::{One, Zero};
    use std::fmt;

    #[derive(Debug)]
    pub struct Array<T> {
        content:Vec<Vec<T>>,
        size:(usize, usize),
    }
    
    impl<T: Clone> Clone for Array<T> {
        fn clone(&self) -> Self {
            Array::<T> {
                content:self.content.clone(),
                size:self.size,
            }
        }
    }
    
    impl<T: fmt::Display> fmt::Display for Array<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut s = String::with_capacity((3 * (self.size.0 + 1) * self.size.1) /* Comma seperated numbers */
            + 2 * (self.size.1 + 1) /* Brackets */ );
        s.push('[');
        for row in 0..self.size.1 {
            s.push('[');
            for col in 0..(self.size.0 - 1) {
                s.push_str(&self.content[row][col].to_string());
                s.push_str(", ");
            }
            s.push_str(&self.content[row][self.size.0 - 1].to_string());
            s.push(']');
            if row != (self.size.1 - 1) {
                s.push_str(", ")
            }
        }
        s.push(']');
        write!(f, "{}", s)
        }
    }

    impl<T: Copy + Clone> Array<T> {
        pub fn new_vec(content:Vec<T>) -> Self {
            let width = 1;
            let height = content.len();
            let content = {
                let mut temp = Vec::<Vec<T>>::with_capacity(height);
                for t in content {
                    temp.push(vec![t]);
                }
                temp
            };
            Array {
                content,
                size:(width, height)
            }
        }

        pub fn new_mat(content:Vec<Vec<T>>) -> Self {
            let height = content.len();
            let width = {
                if height > 0 {
                    content[0].len()
                } else {
                    0
                }
            };
            for v in &content {
                if v.len() != width {
                    panic!("Invalid array initialisation: sub vectors of a matrix must all have equal lengths.");
                }
            }
            Array {
                content,
                size:(width, height),
            }
        }

        pub fn transpose(&self) -> Self {
            let mut content = Vec::<Vec<T>>::with_capacity(self.size.0);
            for col in 0..self.size.0 {
                let mut temp = Vec::<T>::with_capacity(self.size.1);
                for row in 0..self.size.1 {
                    temp.push(self.content[row][col]);
                }
                content.push(temp)
            }
            Array{
                content,
                size:(self.size.1, self.size.0)
            }
        }

        pub fn get_row(&self, index:usize) -> Array<T> {
            if index >= self.size.1 {
                panic!("Index out of bounds: the height is {} but the index is {}", self.size.1, index);
            }

            Array::<T>::new_vec(self.content[index].clone())
        }

        pub fn get_col(&self, index:usize) -> Array<T> {
            if index >= self.size.0 {
                panic!("Index out of bounds: the width is {} but the index is {}", self.size.1, index);
            }
            let mut content = Vec::<T>::with_capacity(self.size.1);
            for row in 0..self.size.1 {
                content.push(self.content[row][index]);
            }

            Array::<T>::new_vec(content)
        }

        pub fn concat_0_axis(a:Self, b:Self) -> Self {
            if a.size.1 != b.size.1 {
                panic!("To concatenate two arrays over the 0-axis they have to be equal in height.");
            }
            let mut content = a.content;
            for row in 0..b.content.len() {
                content[row].append(&mut b.content[row].clone());
            }
            Array {
                content,
                size:(a.size.0 + b.size.0, a.size.1)
            }
        }

        pub fn concat_1_axis(a:Self, b:Self) -> Self {
            if a.size.0 != b.size.0 {
                panic!("To concatenate two arrays over the 1-axis they have to be equal in width.");
            }
            let mut content = a.content;
            for row in b.content {
                content.push(row);
            }
            Array {
                content,
                size:(a.size.0, a.size.1 + b.size.1),
            }
        }
    }

    impl<T: Copy + Clone> Index<(usize, usize)> for Array<T> {
        type Output = T;

        fn index(&self, i:(usize, usize)) -> &Self::Output {
            if i.0 >= self.size.1 {
                panic!("Index out of bounds: the width is {} but the index is {}", self.size.0, i.0);
            } else if i.1 >= self.size.0 {
                panic!("Index out of bounds: the height is {} but the index is {}", self.size.1, i.1);
            }
            &self.content[i.0][i.1]
        }
    }
    
    impl<T: Copy + Clone> IndexMut<(usize, usize)> for Array<T> {
        fn index_mut(&mut self, i:(usize, usize)) -> &mut Self::Output {
            if i.0 >= self.size.1 {
                panic!("Index out of bounds: the width is {} but the index is {}", self.size.0, i.0);
            } else if i.1 >= self.size.0 {
                panic!("Index out of bounds: the height is {} but the index is {}", self.size.1, i.1);
            }
            &mut self.content[i.0][i.1]
        }
    }

    impl<T: Copy + Clone + Add<Output = T>> Add for Array<T> {
        type Output = Array<T>;
        fn add(self, other:Self) -> Self {
            if self.size != other.size {
                panic!("To add two arrays their sizes must be equal");
            }
            let mut content = Vec::<Vec<T>>::with_capacity(self.size.1);
            for row in 0..self.size.1 {
                let mut temp = Vec::<T>::with_capacity(self.size.0);
                for col in 0..self.size.0 {
                    temp.push(self.content[row][col] + other.content[row][col]);
                }
                content.push(temp);
            } 
            Array {
                content,
                size:self.size,
            }
        }
    }

    impl<T: Copy + Clone + Sub<Output = T>> Sub for Array<T> {
        type Output = Array<T>;
        fn sub(self, other:Self) -> Self {
            if self.size != other.size {
                panic!("To subtract two arrays their sizes must be equal");
            }
            let mut content = Vec::<Vec<T>>::with_capacity(self.size.1);
            for row in 0..self.size.1 {
            let mut temp = Vec::<T>::with_capacity(self.size.0);
                for col in 0..self.size.0 {
                    temp.push(self.content[row][col] - other.content[row][col]);
                }
                content.push(temp);
            } 
            Array {
                content,
                size:self.size,
            }
        }
    }

    impl<T: Copy + Clone + Add<Output = T> + Mul<Output = T>> Mul<T> for Array<T> {
        type Output = Array<T>;
        fn mul(self, other:T) -> Self {
            let mut content = Vec::<Vec<T>>::with_capacity(self.size.1);
            for row in 0..self.size.1 {
                let mut temp = Vec::<T>::with_capacity(self.size.0);
                for col in 0..self.size.0 {
                    temp.push(self.content[row][col] * other);
                }
                content.push(temp);
            } 
            Array {
                content,
                size:self.size,
            }                
        }
    }

    impl<T: Copy + Clone + Add<Output = T> + Mul<Output = T>> Mul for Array<T> {
        type Output = Array<T>;
        fn mul(self, other:Self) -> Self {
            if self.size.0 != other.size.1 {
                panic!("To multiply two arrays the first width has to be equal to the seconds height.");
            }
            fn multiply_col<T: Copy + Clone + Add<Output = T> + Mul<Output = T>>
            (a:Array<T>, b:Array<T>) -> Array<T> {
                let mut array = a.get_col(0) * b[(0, 0)];
                for i in 1..a.size.0 {
                    array = array + a.get_col(i) * b[(i, 0)];
                }
                array
            }
            let mut array = multiply_col(self.clone(), other.get_col(0));
            for i in 1..other.size.0 {
                array = Array::concat_0_axis(array.clone(), multiply_col(self.clone(), other.get_col(i)));
            }
            array
        }
    }

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
}