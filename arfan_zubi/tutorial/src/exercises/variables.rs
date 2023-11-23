#![allow(unused)]
pub struct Variables;
impl Variables {
    // Variables.
    // A variable can be used only if it has been initialised.
    pub fn exercise_001() {
        // Uninitialised but used, ERROR!
        let x: i32;
        // Uninitialised but also unused, only a Warning! To avoid warning 
        // prepend with underscore let _y: i32;
        let y: i32; 
        // initialise x.
        x = 5;
        assert_eq!(x, 5);
        println!("Success!\nx = {}", x);
    }
    // Use mut to mark a variable as mutable.
    pub fn exercise_002() {
        let mut x: i32 = 1;
        x += 2;
        assert_eq!(x, 3);
        println!("Success!\nx = {}", x);
    }
    // Scope.
    // A scope is the range within the program for which the item is valid.
    pub fn exercise_003() {
        // Outer scope.
        let x: i32 = 10;
        {
            // Inner scope.
            let y: i32 = 5;
            println!("The value of x is {} and the value of y is {}", x, y);
        }
        // Or Tutors example: define y in outer scope.
        // println!("The value of x is {} and the value of y is {}", x, y);
    }
    pub fn exercise_004() {
        fn main() {
            println!("{}, world", define_x());
        }
    
        fn define_x() -> String {
            let x = "hello".to_string();
            x
        }
        main()
        /* Or Tutors example:
        fn main() {
            define_x();
        }
        fn define_x() {
            let x: &str = "hello";
            println!()"{}, world", x);
        }
         */
    }
    // Shadowing
    pub fn exercise_005() {
        let x: i32 = 5;
        {
            let x = 12;
            assert_eq!(x, 12)
        }
        assert_eq!(x, 5);
        let x = 42;
        // Prints 42
        println!("{}", x);
    }
    pub fn exercise_006() {
        let mut x: i32 = 1;
        x = 7;
        // Shadowing and re-binding. 
        // let x = x;
        x += 3;
        
        let y = 4;
        // Shadowing.
        let y = "I can also be bound to text!";
        println!("Success!\n x = {}\n y = {}", x, y);
    }
    // Unused variables.
    pub fn exercise_007() {
        // To avoid warning, prepend with underscore
        let _x = 1;
        // Or, tell compiler to ignore.
        fn main() {
            let x = 1;
            println!("x: {}", x);
        }
        main();
        println!("_x: {}", _x);
    }
    // Destructuring.
    // Can use pattern with let to destructure a tuple to separate values.
    pub fn exercise_008() {
        let (mut x, y) = (1, 2);
        x += 2;
    
        assert_eq!(x, 3);
        assert_eq!(y, 2);
        println!("Success!\n x = {}\n y = {}", x, y);
    }
    // Destructuring assignments.
    // Rust version 1.59 onwards: use tuple, slice, and struct patterns as the left-hand side 
    // of an assignment.
    pub fn exercise_009() {
        let (x, y);
        (x, ..) = (3, 4);
        [.., y] = [1, 2];
        assert_eq!([x, y], [3, 2]);
        println!("Success!\n x = {}\n y = {}", x, y);
    }
}
