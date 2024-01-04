pub mod util {
    use std::mem;

    struct Node<T> {
        value:T,
        degree:usize,
        children:Vec<Node<T>>,
    }

    impl<T: PartialOrd> Node<T> {
        fn get_root(self) -> (T, Vec<Node<T>>) {
            (self.value, self.children)
        }

        fn merge(mut a:Self, mut b:Self) -> Self {
            if a.value > b.value {
                let temp = b;
                b = a;
                a = temp;
            }
            a.children.push(b);
            a.degree = a.degree + 1;
            a
        }
    }

    pub struct FibonacciHeap<T> {
        smallest:usize,
        root_list:Vec<Node<T>>,
    }

    impl<T: PartialOrd> FibonacciHeap<T> {
        pub fn new() -> Self {
            FibonacciHeap {
                smallest:0,
                root_list:Vec::<Node<T>>::new(),
            }
        }

        pub fn get_min(&self) -> Option<&T> {
            if self.root_list.len() == 0 {
                return None
            } else {
                Some(&self.root_list[self.smallest].value)
            }
        }

        pub fn insert(&mut self, value:T) {
            match self.get_min() {
                Some(v) => {
                    if value < *v {
                        self.smallest = self.root_list.len();
                    }
                }
                None => {
                    self.smallest = 0;
                }
            }
            self.root_list.push(
                Node {
                    value,
                    degree:0,
                    children:Vec::<Node<T>>::new(),
                }
            )
        }

        pub fn extract_min(&mut self) -> Option<T> {
            if self.root_list.len() == 0 {
                return None;
            }
            let (value, mut trees) = {
                let temp = self.root_list.swap_remove(self.smallest);
                temp.get_root()
            };
            self.root_list.append(&mut trees);
            let new_size = {
                let phi = 1.61803399;
                f64::ceil(self.root_list.len() as f64 * phi) as usize
            };
            let mut degree_list = Vec::<Option<Node<T>>>::with_capacity(new_size); // TODO: Update capacity
            while degree_list.len() < new_size {
                degree_list.push(None);
            }

            fn insert<T: PartialOrd>(mut item:Node<T>, degree_list:&mut Vec<Option<Node<T>>>) {
                let other = mem::replace(&mut degree_list[item.degree], None);
                if let Some(other) = other {
                    item = Node::merge(item, other);
                    insert(item, degree_list);
                } else {
                    let degree = item.degree;
                    degree_list[degree] = Some(item);
                }
            }

            while let Some(child) = self.root_list.pop() {
                insert(child, &mut degree_list);
            }

            self.root_list = degree_list.into_iter().filter_map(|e| e).collect();
            self.smallest = self.root_list
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.value.partial_cmp(&b.value).unwrap_or(0.cmp(&0)) )
                .unwrap().0;

            Some(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::util::FibonacciHeap;

    #[test]
    fn extract_min_empty() {
        let expected:Option<f64> = None;
        let actual = FibonacciHeap::<f64>::new().extract_min();
        assert_eq!(expected, actual);
    }

    #[test]
    fn extract_min() {
        let expected:Option<usize> = Some(1);
        let actual = {
            let mut temp = FibonacciHeap::<usize>::new();
            temp.insert(2);
            temp.insert(1);
            temp.insert(3);
            temp.extract_min()
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn extract_min_large() {
        let mut priority_queue = FibonacciHeap::<f64>::new();
        priority_queue.insert(10.1);
        priority_queue.insert(11.1);
        priority_queue.insert(5.0);
        assert_eq!(5.0, priority_queue.extract_min().unwrap());
        assert_eq!(10.1, priority_queue.extract_min().unwrap());
        priority_queue.insert(11.2);
        priority_queue.insert(15.1);
        priority_queue.insert(5.1);
        priority_queue.insert(1.1);
        priority_queue.insert(0.00001);
        priority_queue.insert(5000.1);
        priority_queue.insert(5030.1);
        priority_queue.insert(5230.1);
        assert_eq!(0.00001, priority_queue.extract_min().unwrap());
        assert_eq!(1.1, priority_queue.extract_min().unwrap());
        assert_eq!(5.1, priority_queue.extract_min().unwrap());
        assert_eq!(11.1, priority_queue.extract_min().unwrap());
    }
}