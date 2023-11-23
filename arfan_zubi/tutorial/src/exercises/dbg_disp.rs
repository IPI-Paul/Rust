#![allow(unused)]
pub struct DebugDisplay;
impl DebugDisplay {
    pub fn exercise_001() {
        #[derive(Debug)]
        struct Structure(i32);
        println!("{} months in a year.", 12);
        println!("Now {:?} will print!", Structure(3));
    }
    pub fn exercise_002() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }
        let person = Person { name: "Sunface".to_string(), age: 18 };
        println!("{:#?}", person);
    }
    pub fn exercise_003() {
        use std::fmt;
        struct Structure(i32);
        struct Deep(Structure);
        impl fmt::Debug for Deep {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0.0)
            }
        }
        println!("Now {:?} will print!", Deep(Structure(7)));
    }
    pub fn exercise_004() {
        use std::fmt;
        struct Point2D {
            x: f64,
            y: f64,
        }
        impl fmt::Display for Point2D {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Display: {} + {}i", self.x, self.y)
            }
        }
        impl fmt::Debug for Point2D {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Debug: Complex {{ real: {}, imag: {} }}", self.x, self.y)
            }
        }
        let point = Point2D { x: 3.3, y: 7.2 };
        assert_eq!(format!("{}", point), "Display: 3.3 + 7.2i");
        assert_eq!(format!("{:?}", point), "Debug: Complex { real: 3.3, imag: 7.2 }");
        println!("Success!\npoint: {}\npoint: {:?}", point, point);
    }
    pub fn exercise_005() {
        use std::fmt;
        struct List(Vec<i32>);
        impl fmt::Display for List {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let vec = &self.0;
                write!(f, "[")?;
                for (count, v) in vec.iter().enumerate() {
                    if count != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", count, v )?;
                }
                write!(f, "]")
            }
        }
        let v = List(vec![1, 2, 3]);
        assert_eq!(format!("{}", v), "[0: 1, 1: 2, 2: 3]");
        println!("Success!\nv: {}", v);
    }
}