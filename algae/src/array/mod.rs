pub mod array {
    use std::ops::{Index, IndexMut, Add, Sub, Mul};
    use std::fmt;

    #[derive(Debug)]
    pub struct Array<T> {
        pub(in crate::array) content:Vec<Vec<T>>,
        pub size:(usize, usize),
    }
    
    impl<T: Clone> Clone for Array<T> {
        fn clone(&self) -> Self {
            Array::<T> {
                content:self.content.clone(),
                size:self.size,
            }
        }
    }

    impl<T: PartialEq> PartialEq for Array<T> {
        fn eq(&self, other:&Self) -> bool {
            self.size == other.size && self.content == other.content
        }
    }
    impl<T: Eq> Eq for Array<T> {}
    
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

        pub fn split_0_axis(m:Self, col:usize) -> (Self, Self) {
            let mut a = m.clone();
            let b_size = (usize::saturating_sub(m.size.0, col), m.size.1);
            a.size.0 = col;
            let mut content = Vec::<Vec<T>>::with_capacity(m.size.1);
            for row in 0..a.size.1 {
                content.push(
                    a.content[row].split_off(col)
                );
            }
            let b = Array::<T> {
                content,
                size:b_size,
            };
            (a, b)
        }

        pub fn split_1_axis(m:Self, row:usize) -> (Self, Self) {
            let mut a = m.clone();
            let b_size = (m.size.0, usize::saturating_sub(m.size.1, row));
            a.size.1 = row;
            let content = a.content.split_off(row);
            let b = Array::<T> {
                content,
                size:b_size,
            };
            (a, b)
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
}

pub mod methods;