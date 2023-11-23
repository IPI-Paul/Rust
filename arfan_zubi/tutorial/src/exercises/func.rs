#![allow(dead_code, unused)]
pub struct Functions;
impl Functions {
    pub fn exercise_001() {
        let (x, y) = (1, 2);
        let s = sum(x, y);
        assert_eq!(s, 3);
        println!("Success!\nsum(x, y) = {:?}", sum(x, y));
        
        fn sum(x: i32, y: i32) -> i32 {
            x + y
        }
    }
    pub fn exercise_002() {
        print();

        fn print() {
            println!("Success!")
        }
    }
    pub fn exercise_003() {
        never_return();
        println!("Failed!");

        // Diverging function never return to the caller and may be used in places where a value of 
        // any type is expected.
        fn never_return() -> ! {
            // Implement this function, don't modify the fn signatures.
            panic!("Stop programme!");
        }
    }
    pub fn exercise_004() {
        println!("Success!");

        fn get_option(tp: u8) -> Option<i32> {
            match tp {
                1 => {
                    todo!()
                },
                _ => {
                    todo!()
                }
            };
            never_return()
        }
        fn never_return() -> ! {
            panic!("Stop programme!");
            unimplemented!();
            todo!()
        }
    }
    pub fn exercise_005() {
        let b = false;
        let v = match b {
            true => 1,
            // Diverging functions can also be used in match expressions to replace a value.
            false => {
                println!("Success!");
                panic!("We have no value for false, but we can panic");
            }
        };
        println!("Exercise Failed if printing oput this line!");
    }
}
