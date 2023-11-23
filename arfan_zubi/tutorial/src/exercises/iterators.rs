
pub struct Iterators;
impl Iterators {
    pub fn exercise_001() {
        let arr = [0; 10];
        for i in arr {
            print!("{} ", i);
        }
    }
    pub fn exercise_002() {
        let mut v = Vec::new();
        for n in 0..100 {
            v.push(n)
        }
        assert_eq!(v.len(), 100);
        println!("v.len(): {}", v.len());
    }
    pub fn exercise_003() {
        let mut v1 = vec![1, 2].into_iter();
        assert_eq!(v1.next(), Some(1));
        assert_eq!(v1.next(), Some(2));
        assert_eq!(v1.next(), None);
        println!("Success!");
    }
    pub fn exercise_004() {
        let arr = vec![0; 10];
        for i in arr.iter() {
            println!("{}", i);
        }
        println!("{:?}", arr);
    }
    pub fn exercise_005() {
        let mut names = vec!["Bob", "Frank", "Ferris"];
        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }
        println!("names: {:?}", names);
    }
    pub fn exercise_006() {
        let mut values = vec![1, 2, 3];
        let mut values_iter = values.iter_mut();
        if let Some(v) = values_iter.next() {
            *v = 0
        }
        assert_eq!(values, vec![0, 2, 3]);
        println!("values: {:?}", values);
    }
    pub fn example_001() {
        #[derive(Debug)]
        struct Counter {
            count: u32,
        }
        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }
        impl Iterator for Counter {
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                if self.count < 5 {
                    self.count += 1;
                    Some(self.count)
                } else {
                    None
                }
            }
        }
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
        println!(
            "Counter::new(): {:?}\nCounter::new().next(): {:?}", 
            Counter::new(), Counter::new().next()
        );
    }
    pub fn exercise_007() {
        #[derive(Debug)]
        struct Fibonacci {
            curr: u32,
            next: u32,
        }
        impl Iterator for Fibonacci {
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                if self.curr + self.next < u32::MAX {
                    let forward = self.curr + self.next;
                    self.curr = self.next;
                    self.next = forward;
                    Some(self.curr)
                } else {
                    None
                }
            }
        }
        fn fibonnaci() -> Fibonacci {
            Fibonacci { curr: 0, next: 1 }
        }
        let mut fib = fibonnaci();
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
        println!("{:?}", fib.next());
    }
    pub fn exercise_008() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
        println!("v1: {:?}\ntotal: {}", v1, total);
    }
    pub fn exercise_009() {
        use std::collections::HashMap;
        let names = [("sunface", 18), ("sunfei", 18)];
        let folks: HashMap<&str, i32> = names.into_iter().collect();
        println!("{:?}", folks);
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<i32> = v1.into_iter().collect();
        assert_eq!(v2, vec![1, 2, 3]);
        println!("names: {:?}\nfolks: {:?}\nv2: {:?}", names, folks, v2);
    }
    pub fn exercise_010() {
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
        println!("v1: {:?}\nv2: {:?}", v1, v2);
    }
}
