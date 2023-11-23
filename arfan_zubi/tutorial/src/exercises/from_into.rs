#![allow(unused)]
pub struct FromIntoConversion;
impl FromIntoConversion {
    // When implementing From trait, into trait is automatically implemented.
    pub fn exercise_001() {
        let i1: i32 = false.into();
        let i2: i32 = i32::from(false);
        assert_eq!(i1, i2);
        assert_eq!(i1, 0);
        let i3: u32 = 'a'.into();
        let i3 = 'a' as i32;
        let s: String = 'a'.to_string();
        let s: String = String::from('a');
        let s: String = 'a'.into();
        println!("Success!\ni1: {}\ni2: {}\ni3: {}\ns: '{}'", i1, i2, i3, s);
    }
    pub fn exercise_002() {
        #[derive(Debug)]
        struct Number {
            value: i32,
        }
        impl From<i32> for Number {
            fn from(value: i32) -> Self {
                Self { value }
            }
        }
        let num = Number::from(30);
        assert_eq!(num.value, 30);
        let num: Number = 30.into();
        assert_eq!(num.value, 30);
        println!("Success!\nnum: {:?}", num);
    }
    pub fn exercise_003() {
        use std::{fs, io, num};
        #[derive(Debug)]
        enum CliError {
            IoError(io::Error),
            ParseError(num::ParseIntError),
        }
        impl From<io::Error> for CliError {
            fn from(value: io::Error) -> Self {
                Self::IoError(value)
            }
        }
        impl From<num::ParseIntError> for CliError {
            fn from(value: num::ParseIntError) -> Self {
                Self::ParseError(value)
            }
        }
        fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
            // ? auomaiclly converts io::Error to CliError.
            let contents = fs::read_to_string(&file_name)?;
            let num: i32 = contents.trim().parse()?;
            Ok(num)
        }
        println!("Success!\nFile not found: {:?}", open_and_parse_file("file_name"));
    }
    pub fn exercise_004() {
        let n: i16 = 256;
        let n: u8 = match n.try_into() {
            Ok(n) => n,
            Err(e) => {
                println!("there is an error when converting: {}, but we catch it", e.to_string());
                0
            }
        };
        assert_eq!(n, 0);
        println!("Success!\nn: {}", n);
    }
    pub fn exercise_005() {
        #[derive(Debug, PartialEq)]
        struct EvenNum(i32);
        impl TryFrom<i32> for EvenNum {
            type Error = ();
            fn try_from(value: i32) -> Result<Self, Self::Error> {
                if value % 2 == 0 {
                    Ok(EvenNum(value))
                } else {
                    Err(())
                }
            }
        }
        assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
        assert_eq!(EvenNum::try_from(5), Err(()));
        let result: Result<EvenNum, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNum(8)));
        let result: Result<EvenNum, ()> = 5i32.try_into();
        assert_eq!(result, Err(()));
        println!("Success\nresult: {:?}", result)
    }
}
