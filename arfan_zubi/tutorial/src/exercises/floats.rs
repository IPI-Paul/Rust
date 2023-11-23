pub struct FloatingPoint;
impl FloatingPoint {
    pub fn exercise_001() {
        let x = 1_000.000_1;
        let y: f32 = 0.12;
        let z = 0.01_f64;

        assert_eq!(type_of(&x), "f64".to_string());
        println!(
            "Success!\nType of x is {} and = {}\nType of y is {} and = {}\nType of z is {} and = {}",
            type_of(&x), x, type_of(&y), y, type_of(&z), z
        );
        // Get the type of a given variable and return a string representation of the type.
        fn type_of<T>(_: &T) -> String {
            format!("{}", std::any::type_name::<T>())
        }
    }
    pub fn exercise_002() {
        // Panic caused by floating point precision. Implicitly uses f64.
        // assert!(0.1 + 0.2 == 0.3);
        assert!(0.1_f32 + 0.2 == 0.3);
        // Or.
        assert!(0.1 as f32 + 0.2 == 0.3);
        println!("Success!\n{} + {} = {}", 0.1, 0.2, 0.3);
    }
    pub fn exercise_003() {
        let mut sum = 0;
        for i in -3..2 {
            println!("{} + {} = {}", sum, i, sum + i);
            sum += i
        }
        println!("");
        assert!(sum == -5);
        for c in 'a'..='z' {
            print!("{} ", c);
        }
        println!("");
        for c in 'a'..='z' {
            print!("{} ", c as u8);
        }
    }
    pub fn exercise_004() {
        use std::ops::{Range, RangeInclusive};
        assert_eq!((1..5), Range{ start: 1, end: 5 });
        assert_eq!((1..=5), RangeInclusive::new(1, 5));
        println!("Success!")
    }
}
