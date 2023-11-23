#![allow(unused)]
pub struct MethodsAndFuncions;
impl MethodsAndFuncions {
    pub fn exercise_001() {
        #[derive(Debug, Clone, Copy)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        impl Rectangle {
            fn area(self) -> u32 {
                self.width * self.height
            }
        }
        let rect1 = Rectangle { width: 30, height: 50 };
        assert_eq!(rect1.area(), 1500);
        println!("Success!\nrect1: {:?}\nrect1.area: {}", rect1, rect1.area());
    }
    pub fn exercise_002() {
        #[derive(Debug)]
        struct TrafficLight {
            color: String,
        }
        impl TrafficLight {
            pub fn show_state(self: &Self) {
                println!("the current state is {}", self.color);
            }
            pub fn change_state(&mut self) {
                self.color = "green".to_string()
            }
        }
        let mut tl = TrafficLight {color: "red".to_string() };
        tl.show_state();
        tl.change_state();
        println!("Success!\ntl: {:?}", tl);
    }
    pub fn exercise_004() {
        #[derive(Debug)]
        struct TrafficLight {
            color: String,
        }
        impl TrafficLight {
            pub fn new() -> Self {
                Self { color: "red".to_string() }
            }
            pub fn get_state(&self) -> &str {
                &self.color
            }
        }
        let light = TrafficLight::new();
        assert_eq!(light.get_state(), "red");
        println!("Success!\nlight: {:?}", light);
    }
    pub fn exercise_005() {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
            // fn can_hold(&self, other: &Rectangle) -> bool {
            //     self.width > other.width && self.height > other.height
            // }
            // Or
        }
        impl Rectangle {
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }
        let rect1 = Rectangle { width: 5, height: 5};
        let rect2 = Rectangle { width: 10, height: 10};
        println!(
            "Success!\nrect1: {:?} with an are of {}\nrect2: {:?} with an area of {} can hold rect1 = {}",
            rect1, rect1.area(), rect2, rect2.area(), rect2.can_hold(&rect1)
        );
    }
    pub fn exercise_006() {
        #[derive(Debug)]
        enum TrafficLightColor {
            Red, 
            Yellow,
            Green,
        }
        impl TrafficLightColor {
            fn color(&self) -> &str {
                match self {
                    Self::Red => "red",
                    Self::Yellow => "yellow",
                    Self::Green => "green",
                }                
            }
        }
        let c = TrafficLightColor::Yellow;
        assert_eq!(c.color(), "yellow");
        println!("{:?}", c);
    }
}
