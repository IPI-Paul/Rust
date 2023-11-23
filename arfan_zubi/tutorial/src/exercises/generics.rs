#![allow(unused)]
pub struct Generics;
impl Generics {
    pub fn exercise_001() {
        struct A;
        struct S(A);
        struct SGen<T>(T);

        fn reg_fn(_s: S) {}
        fn gen_spec_t(_s: SGen<A>) {}
        fn gen_spec_i32(_s: SGen<i32>) {}
        fn generic<T>(_s: SGen<T>) {}

        reg_fn(S(A));
        gen_spec_t(SGen(A));
        gen_spec_i32(SGen(7));
        generic::<char>(SGen('A'));
        generic(SGen('Z'));

        println!("Success!");
    }
    pub fn exercise_002() {
        // T must implement the Add trait in order to use +.
        fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
            a + b
        }
        assert_eq!(5, sum(2i8, 3i8));
        assert_eq!(50, sum(20, 30));
        assert_eq!(2.46, sum(1.23, 1.23));
        println!("Success!")
    }
    pub fn exercise_003() {
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }

        let integer = Point {x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
        println!("Success!\ninteger: {:?}\nfloat: {:?}", integer, float);
    }
    pub fn exercise_004() {
        #[derive(Debug)]
        struct Point<T, U> {
            x: T,
            y: U
        }

        let p = Point { x: 5, y: "hello".to_string() };
        println!("Success!\np: {:?}", p)
    }
    pub fn exercise_005() {
        struct Val<T> {
            val: T,
        }
        impl<T> Val<T> {
            fn value(&self) -> &T {
                &self.val
            }
        }
        let x = Val { val: 3.0 };
        let y = Val { val: "hello". to_string() };
        println!("x: {}\ny: {}", x.value(), y.value());
    }
    pub fn exercise_006() {
        #[derive(Debug, Clone, Copy)]
        struct  Point<T, U> {
            x: T,
            y: U,
        }
        impl<T, U> Point<T, U> {
            fn mixup<V, W>(self, other: Point <V, W>) -> Point<T, W> {
                Point { x: self.x, y: other.y }
            }
        }
        let p1 = Point { x: 5, y: 10 };
        let p2 = Point { x: "Hello", y: '中' };
        let p3 = p1.mixup(p2);
        assert_eq!(p3.x, 5);
        assert_eq!(p3.y, '中');
        println!("Success!\np1: {:?}\np2: {:?}\np3: {:?}", p1, p2, p3);
    }
    pub fn exercise_007() {
        struct Point<T> {
            x: T,
            y: T,
        }
        impl Point<f64> {
            fn distance_from_origin(&self) -> f64 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
        let p = Point { x: 5.0, y: 10.0 };
        println!("p.distance_from_origing: {}", p.distance_from_origin())
    }
}
