pub struct Patterns;
impl Patterns {
    pub fn exercise_001() {
        fn match_number(n: i32) {
            match n {
                1 => println!("One"),
                2 | 3 | 4 | 5 => println!("match 2 -> 5"),
                6..=10 => println!("match 6 -> 10"),
                _ => println!("match -infinite -> 0 or 11 to +infinite")
            }
        }
        match_number(9);
    }
    pub fn exercise_002() {
        struct Point1 {
            x: i32,
            y: i32,
        }
        let p = Point1 {x: 3, y: 30 };
        match p {
            Point1 { x, y: 0 } => println!("On the x axis at {}", x),
            // @ destructures y into y variable.
            Point1 { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
            Point1 { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }
    pub fn exercise_003() {
        enum Message {
            Hello { id: i32 },
        }
        let msg = Message::Hello { id: 5 };
        match msg {
            Message::Hello { 
                id: id @ 3..=7 
            }  => println!("Found an id in the range [3, 7]: {}", id),
            Message::Hello { id: newid@ (10 | 11 | 12) } => {
                println!("Found an id in another range [10, 12]: {}", newid)
            },
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }
    pub fn exercise_004() {
        let num = Some(4);
        let split = 5;
        match num {
            // Some(x) destructures x, but if condition is a match guard.
            Some(x) if x < split => assert!(x < split),
            Some(x) => assert!(x >= split),
            None => ()
        }
        println!("Success!\nnum: {}", num.unwrap());
    }
    pub fn exercise_005() {
        let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);
        match numbers {
            (first, .., last) => {
                assert_eq!(first, 2);
                assert_eq!(last, 2048);
            }
        }
        println!("Success!\nnumbers: {:?}", numbers);
    }
    pub fn exercise_006() {
        let mut v = String::from("hello,");
        let r = &mut v;
        match r {
            value => {
                value.push_str(" world!");
                println!("value: {}", value);
            }
        }
    }
}
