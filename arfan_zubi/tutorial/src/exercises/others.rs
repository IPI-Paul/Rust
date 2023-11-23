#![allow(unused)]
pub struct Others;
impl Others {
    pub fn exercise_001() {
        use std::fmt;
        struct Point {
            x: i32,
            y: i32,
        }
        impl fmt::Display for Point {
            // lifetime not needed. f is the buffer.
            // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "The point is ({}, {})", self.x, self.y)
            }
        }
        let origin = Point {x: 0, y: 0};
        assert_eq!(origin.to_string(), "The point is (0, 0)");
        assert_eq!(format!("{}", origin), "The point is (0, 0)");
        println!("Success!\norigin: {}", origin);
    }
    pub fn exercise_002() {
        use std::str::FromStr;
        let parsed: i32 = "5".parse().unwrap();
        let turbo_parsed = "10".parse::<i32>().unwrap();
        let from_str: i32 = FromStr::from_str("20").unwrap();
        // Or
        let from_str = i32::from_str("20").unwrap();
        let sum = parsed + turbo_parsed + from_str;
        assert_eq!(sum, 35);
        println!(
            "Success!\nparsed: {}\nturbo_parsed: {}\nfrom_str: {}\nsum: {}",
            parsed, turbo_parsed, from_str, sum
        );
    }
}
