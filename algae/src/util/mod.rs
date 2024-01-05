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
            if self.root_list.len() > 0 {
                self.smallest = self.root_list
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.value.partial_cmp(&b.value).unwrap_or(0.cmp(&0)) )
                .unwrap().0;
            }

            Some(value)
        }
    }

    pub struct PrimeCalculator {
        primes:Vec<usize>,
    }

    impl PrimeCalculator {
        pub fn new() -> Self {
            PrimeCalculator {
                primes:vec![2],
            }
        }

        fn extend_upto(&mut self, num:usize) {
            let mut last = *(&self.primes).last().unwrap_or(&0);
            while last * last < num {
                last = self.next(last);
                self.primes.push(last);
            }
        }

        pub fn is_prime(&mut self, num:usize) -> bool {
            self.extend_upto(num);
            !(self.primes.iter().any(|p| *p != num && num % p == 0) || num == 1)
        }

        pub fn next(&mut self, mut num:usize) -> usize {
            num = num + 1;
            self.extend_upto(num);
            while !self.is_prime(num) {
                num += 1;
            }
            num
        }
    }

    pub struct IdManager {
        height_index:Vec<Vec<usize>>,
        prime_calculator:PrimeCalculator,
        height:usize,
        node_amnt:usize,
    }

    impl IdManager {
        fn expand_height_index(height_index:&mut Vec<Vec<usize>>, prime_calculator:&mut PrimeCalculator, node_amnt:usize) {
            let mut temp = Vec::<usize>::with_capacity(node_amnt);
            let mut last = **(&(&height_index).last().unwrap_or(&vec![0]).last().unwrap_or(&0));
            while temp.len() < node_amnt {
                last = prime_calculator.next(last);
                temp.push(last);
            }
            height_index.push(temp);
        }

        pub fn new(node_amnt:usize) -> Self {
            let mut prime_calculator = PrimeCalculator::new();
            let mut height_index = Vec::<Vec<usize>>::with_capacity(1);
            IdManager::expand_height_index(&mut height_index, &mut prime_calculator, node_amnt);
            IdManager {
                height_index,
                prime_calculator,
                height:0,
                node_amnt,
            }
        }

        pub fn get_primes(&mut self) -> &Vec<usize> {
            while self.height_index.len() <= self.height {
                Self::expand_height_index(&mut self.height_index, &mut self.prime_calculator, self.node_amnt)
            }
            &self.height_index[self.height]
        }

        pub fn move_down(&mut self) {
            self.height += 1;
        }

        pub fn set_height(&mut self, height:usize) {
            self.height = height;
        }
    }
    
    pub struct PrimeNode<T> {
        pub value:T,
        key:usize,
        height:usize,
    }

    impl<T: Copy + Clone> PrimeNode<T> {
        pub fn new_root(value:T) -> Self {
            PrimeNode {
                value,
                key:1,
                height:0,
            }
        }

        pub fn generate_children(&self, values:Vec<T>, id_manager:&mut IdManager) -> Vec<Self> {
            let mut children = Vec::<PrimeNode<T>>::with_capacity(id_manager.node_amnt);
            id_manager.set_height(self.height);
            let primes = id_manager.get_primes();
            for i in 0..usize::min(values.len(), primes.len()) {
                children.push( PrimeNode {
                    value:values[i],
                    key:self.key * primes[i],
                    height:self.height + 1,
                });
            }
            children
        }

        pub fn is_child(&self, other:&Self) -> bool {
            self.key % other.key == 0
        }

        pub fn is_parent(&self, other:&Self) -> bool {
            other.key % self.key == 0
        }

        pub fn shares_generation(&self, other:&Self) -> bool {
            self.height == other.height
        }
    }
    // IDK why I made this data-structure. 
    // With this you could hand around these types of PrimeNodes and transfer ownership as you wish, 
    // without loosing knowledge of whether something is a child of another.
}

#[cfg(test)]
mod tests {
    use crate::util::util::FibonacciHeap;
    use crate::util::util::PrimeCalculator;
    use crate::util::util::IdManager;
    use crate::util::util::PrimeNode;

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

    #[test]
    fn is_prime() {
        let mut prime_calc = PrimeCalculator::new();
        assert!(!prime_calc.is_prime(30));
        assert!(prime_calc.is_prime(2));
        assert!(prime_calc.is_prime(5));
        assert!(!prime_calc.is_prime(18));
        assert!(prime_calc.is_prime(7));
        assert!(prime_calc.is_prime(3));
        assert!(prime_calc.is_prime(11));
        assert!(!prime_calc.is_prime(12));
    }

    #[test]
    fn next_prime() {
        let mut prime_calc = PrimeCalculator::new();
        assert_eq!(13, prime_calc.next(11));
        assert_eq!(5, prime_calc.next(3));
        assert_eq!(2, prime_calc.next(0));
        assert_eq!(7, prime_calc.next(5));
    }

    #[test]
    fn id_manager_one() {
        let mut id_manager = IdManager::new(3);
        id_manager.move_down();
        id_manager.move_down();
        id_manager.move_down();
        let expected = vec![29, 31, 37];
        let actual = id_manager.get_primes();
        for i in 0..actual.len() {
            assert_eq!(expected[i], actual[i])
        }
    }

    #[test]
    fn id_manager_two() {
        let mut id_manager = IdManager::new(10);
        id_manager.move_down();
        id_manager.move_down();
        let expected = vec![73, 79, 83, 89, 97, 101, 103, 107, 109, 113];
        let actual = id_manager.get_primes();
        for i in 0..actual.len() {
            assert_eq!(expected[i], actual[i])
        }
    }

    #[test]
    fn prime_nodes() {
        let mut id_manager = IdManager::new(6);
        let god_father = PrimeNode::new_root(100);
        let children = god_father.generate_children(vec![5, 3, 2, 6], &mut id_manager);
        let grand_children = children[3].generate_children(vec![5, 3, 7, 20, 6], &mut id_manager);
        assert!(grand_children[3].is_child(&children[3]));
        assert!(god_father.is_parent(&children[3]));
    }
}