#![allow(unused)]
pub struct ExpressionStatement;
impl ExpressionStatement {
    pub fn exercise_001() {
        let x = 5u32;
        let y = {
            let x_squared = x * x;
            let x_cube = x_squared * x;
            // This expression will be assigned to y.
            x_cube + x_squared + x
        };
        let z = {
            // The semicolon suppresses this expression and () is assigned to z.
            2 * x;
        };
        println!("x is {:?}", x);
        println!("y is {:?}", y);
        println!("z is {:?}", z);
    }
    pub fn exercise_002() {
        let v = {
            let mut x = 1;
            x += 2
        };
        assert_eq!(v, ());
        println!("Success!\nv = {:?}", v);
        let v = {
            let mut x = 1;
            x += 2;
            x
        };
        assert_eq!(v, 3);
        println!("v = {}", v);
    }
    pub fn exercise_003() {
        // let v = (let x = 3);
        let v = {
            let x = 3; 
            x
        };
        assert!(v == 3);
        println!("Success!\nv = {}", v);
    }
    pub fn exercise_004() {
        let s = sum(1, 2);
        assert_eq!(s, 3);
        println!("Success!\ns = {}", s);
        
        fn sum(x: i32, y: i32) -> i32 {
            x + y
        }
    }
}
