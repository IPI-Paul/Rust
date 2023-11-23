#![allow(unused)]
pub struct Panics;
impl Panics {
    pub fn exercise_001() {
        fn drink(beverage: &str) {
            if beverage == "lemonade" {
                println!("Success!");
                panic!("panic!() clears the stack before exiting!");
            }
            println!("Exercise Failed if printing out this line!");
        }
        drink("lemonade");
        println!("Exercise Failed if printing out this line!");
    }
    pub fn exercise_002() {
        assert_eq!("abc".as_bytes(), [97, 98, 99]);
        let v = vec![1, 2, 3];
        let ele = v[2];
        let ele = v.get(1).unwrap();
        let v = production_rate_per_hour(2);
        divide(15, 1);
        println!(
            "Success!\ndivide_my(15, 2) using generic T: {}\
            \ndivide_my_f32(15u16, 2) using generic T: {}\
            \ndivide_my_f32(15.06, 2.0) using generic T: {}\
            \ndivide_my_f32('z' as u8, 'b' as u8) using generic T: {}", 
            divide_my(15, 2), divide_my_f32(15u16, 2), divide_my_f32(15.06, 2.0), 
            divide_my_f32('z' as u8, 'b' as u8)
        );

        fn divide_my<
            T: Into<i32> + std::ops::Div<Output = T> + std::clone::Clone
        >(x: T, y: T) -> i32 {
            if (x.clone().into() == 0) | (y.clone().into() == 0) {
                return 0;
            }
            (x / y).into()
        }
        fn divide_my_f32<T: Into<f32> + std::clone::Clone>(x: T, y: T) -> f32 {
            if (Into::<f32>::into(x.clone()) == 0.0) | (Into::<f32>::into(y.clone()) == 0.0) {
                return 0.0;
            }
            Into::<f32>::into(x) / Into::<f32>::into(y)
        }
        fn divide(x: u8, y: u8) {
            println!("{}", x / y)
        }
        fn production_rate_per_hour(speed: u16) -> f64 {
            let cph: u16 = 221;
            match speed {
                1..=4 => (speed * cph) as f64,
                5..=8 => (speed * cph) as f64 * 0.9,
                9..=10 => (speed * cph) as f64 * 0.77,
                _ => 0 as f64,
            }
        }
        fn working_items_per_minute(speed: u16) -> u32 {
            (production_rate_per_hour(speed) /60 as f64) as u32
        }
    }
}
