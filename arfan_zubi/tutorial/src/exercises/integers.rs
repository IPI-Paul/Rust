#![allow(unused)]
pub struct Integers;
impl Integers {
    // arc (isize and usize) = Computers architechture.
    pub fn exercise_001() {
        let x: i32 = 5;
        let mut y = 5;
        y = x;
        /* Or:
        let mut y: u32 = 5;
        y = x as u32;
         */
        let z = 10;
        println!("Success!\nx = {}\ny = {}\nz = {}", x, y, z);
    }
    pub fn exercise_002() {
        let v: u16 = 38_u8 as u16;
        println!("Success!\nv as u16 = {} as u8 cast as u16", v);
    }
    // If a type is not explicitly assigned to a variable the compiler will infer one.
    pub fn exercise_003() {
        let x = 5;
        // Google searched.
        // fn type_of<T>(_: T) -> &'static str {
        //     type_name::<T>()
        // }
        /* Tutor changed annotation: 
            let x: u32 = 5;
            assert_eq!("u32".to_string(), type_of(&x));
        */
        assert_eq!("i32".to_string(), type_of(&x));
        println!("Success!\nType of x is {}",type_of(&x));
        // Get the type of a given variable and return a string representation of the type.
        fn type_of<T>(_: &T) -> String {
            format!("{}", std::any::type_name::<T>())
        }
    }
    pub fn exercise_004() {
        assert_eq!(i8::MAX, 127);
        assert_eq!(u8::MAX, 255);
        println!("Success!\ni8::MAX = {}\nu8::MAX = {}", i8::MAX, u8::MAX)
    }
    pub fn exercise_005() {
        let v1 = 251_u16 + 8;
        let v2 = i16::checked_add(251, 8).unwrap();
        println!("{}, {}", v1, v2);
    }
    pub fn exercise_006() {
        //          decimal  hexa decimal 255  octal 63      binary 255.
        let v = 1_024 +        0xff +       0o77 +    0b1111_1111;
        assert!(v == 1597);
        println!("Success!\nv = {}", v);
    }
}
