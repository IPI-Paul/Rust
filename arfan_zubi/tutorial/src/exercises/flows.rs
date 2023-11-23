#![allow(unused)]
pub struct ControlFlow;
impl ControlFlow {
    pub fn exercise_001() {
        let n = 5;
        if n < 0 {
            println!("{} is negative", n);
        } else if n > 0 {
            println!("{} is positive", n);
        } else {
            println!("{} is zero", n)
        }
    }
    pub fn exercise_002() {
        let n = 5;
        let big_n = 
            if n < 10 && n > -10 {
                println!(", and is a small number, increase ten-fold");
                10 * n
            } else {
                println!(", and is a big number, halve the number");
                // n / 2
                n / 2.0 as i32
            };
        println!("{} -> {}", n, big_n);
    }
    pub fn exercise_003() {
        for n in 1..100 {
            if n == 100 {
                panic!("NEVER LE THIS RUN")
            }
        }
        println!("Success!");
    }
    pub fn exercise_004() {
        let names = [String::from("liming"), String::from("hanmeimei")];
        for name in &names {
            println!("name: '{}'", name);
        }
        println!("names: {:?}", names);
        let numbers = [1, 2, 3];
        for n in numbers {
            println!("n: {}", n);
        }
        println!("numbers: {:?}", numbers);
    }
    pub fn exercise_005() {
        let a = [4, 3, 2, 1];
        for (i, v) in a.iter().enumerate() {
            println!("The {}th element is {}", i + 1, v);
        }
    }
    pub fn exercise_006() {
        let mut n = 1;
        while n < 10 {
            if n % 15 == 0 {
                println!("fixxbuzz");
            } else if n % 3 == 0 {
                println!("fizz")
            } else if n % 5 == 0 {
                println!("buzz")
            } else {
                println!("{}", n);
            }
            n += 1;
        }
        println!("n reached {}, so loop is over", n);
    }
    pub fn exercise_007() {
        let mut n = 0;
        for i in 0..=100 {
            if n == 66 {
                break;
            }
            n += 1
        }
        assert_eq!(n, 66);
        println!("Success!\nn: {}", n);
    }
    pub fn exercise_008() {
        let mut n = 0;
        for i in  0..=100 {
            if n != 66 {
                n += 1;
                continue;
            }
            break;
        }
        assert_eq!(n, 66);
        println!("Success!\nn: {}", n);
    }
    pub fn exercise_009() {
        let mut count = 0u32;
        println!("Let's count until infinity!");
        loop {
            count += 1;
            if count == 3 { 
                println!("three");
                continue;
            }
            println!("{}", count);
            if count == 5 {
                println!("OK, that's enough!");
                break;
            }
        }
        assert_eq!(count, 5);
        println!("Success!\ncount: {}", count);
    }
    pub fn exercise_010() {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        assert_eq!(result, 20);
        println!("Success!\nresult: {}", result);
    }
    pub fn exercise_011() {
        let mut count = 0;
        'outer: loop {
            'inner1: loop {
                if count >= 20 {
                    break 'inner1;
                }
                count += 2;
            }
            count += 5;
            'inner2: loop {
                if count >= 30 {
                    break 'outer;
                }
                continue 'outer;
            }
        }
        assert!(count == 30);
        println!("count: {}", count);
    }
}
