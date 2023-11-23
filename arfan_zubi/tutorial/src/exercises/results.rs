#![allow(unused)]
pub struct Results;
impl Results {
    pub fn exercise_001() {
        use std::num::ParseIntError;
        fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
            let n1 = n1_str.parse::<i32>();
            let n2 = n2_str.parse::<i32>();
            Ok(n1.unwrap() * n2.unwrap())
        }
        let result = multiply("10", "2");
        assert_eq!(result, Ok(20));
        println!("result: {}", result.unwrap());
        let result = multiply(&('t' as u8).to_string(), "2");
        assert_eq!(result.as_ref().unwrap(), &232);
        println!("result: {}", result.unwrap());
        let result = multiply("4", "2");
        assert_eq!(result.as_ref().unwrap(), &8);
        println!("Success!\nresult: {}", result.unwrap());
    }
    pub fn exercise_002() {
        use std::num::ParseIntError;
        fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
            let n1 = n1_str.parse::<i32>()?;
            let n2 = n2_str.parse::<i32>()?;
            Ok(n1 * n2)
        }
        assert_eq!(multiply("3", "4").unwrap(), 12);
        println!("Success!\nmultiply(\"3\", \"4\").unwrap(): {}", multiply("3", "4").unwrap());
    }
    pub fn exercise_003() {
        use std::{fs::File, io::{self, Read, Error}};
        fn read_file1() -> Result<String, Error> {
            let f = File::open("hello.txt");
            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };
            let mut s = String::new();
            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }
        fn read_file2() -> Result<String, Error> {
            let mut s = String::new();
            File::open("hello.txt")?.read_to_string(&mut s)?;
            Ok(s)
        }
        assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
        println!(
            "Success!\nread_file1().unwrap_err().to_string(): {}", 
            read_file1().unwrap_err().to_string()
        )
    }
    pub fn exercise_004() {
        use std::num::ParseIntError;
        fn add_two_map(n_str: &str) -> Result<i32, ParseIntError> {
            n_str.parse::<i32>().map(|i| i + 2)
        }
        fn add_two_then(n_str: &str) -> Result<i32, ParseIntError> {
            n_str.parse::<i32>().and_then(|i| Ok(i + 2))
        }
        assert_eq!(add_two_map("4").unwrap(), 6);
        assert_eq!(add_two_then("4").unwrap(), 6);
        println!(
            "Success!\nadd_two_map(\"4\").unwrap(): {}\nadd_two_then(\"4\").unwrap(): {}", 
            add_two_map("4").unwrap(), add_two_then("4").unwrap()
        );
    }
    pub fn exercise_005() {
        use std::num::ParseIntError;
        fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
            match n1_str.parse::<i32>() {
                Ok(n1) => {
                    match n2_str.parse::<i32>() {
                        Ok(n2) => {
                            Ok(n1 * n2)
                        },
                        Err(e) => Err(e),
                    }
                },
                Err(e) => Err(e),
            }
        }
        fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
            n1_str
                .parse::<i32>()
                .and_then(
                    |n1| 
                    n2_str.parse::<i32>().map(|n2| n1 * n2)
                )
        }
        fn print(result: Result<i32, ParseIntError>) {
            match result {
                Ok(n) => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }
        let twenty = multiply1("10", "2");
        print(twenty);
        let tt = multiply("t", "2");
        print(tt);
        println!(
            "Success!\nmultiply(\"2\", \"3\").unwrap(): {}\nmultiply1(\"2\", \"3\").unwrap(): {}",
            multiply("2", "3").unwrap(), multiply1("2", "3").unwrap()
        );
    }
    pub fn exercise_006() {
        use std::num::ParseIntError;
        type Res<T> = Result<T, ParseIntError>;
        fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
            first_number_str.parse::<i32>().and_then(|first_number| {
                second_number_str
                    .parse::<i32>()
                    .map(|second_number| first_number * second_number)
            })
        }
        fn print(result: Res<i32>) {
            match result {
                Ok(n) => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }
        print(multiply("10", "2"));
        print(multiply("t", "2"));
        println!("Success!")
    }
    // main can also return a Result type which also allows the use of ? operator.
}
