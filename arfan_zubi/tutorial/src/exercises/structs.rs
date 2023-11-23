#![allow(unused)]
pub struct Structs;
impl Structs {
    pub fn exercise_001() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
            hobby: String,
        }
        let age = 30;
        let p = Person {
            name: String::from("sunface"),
            age,
            hobby: String::from("coding"),
        };
        println!("Success!\np: {:?}", p);
    }
    pub fn exercise_002() {
        #[derive(Debug)]
        struct Unit;
        trait SomeTrait {
            // ..Some behaviours defined here.
        }
        impl SomeTrait for Unit {}
        fn do_something_with_unit(u: &Unit) {}

        let u = Unit;
        do_something_with_unit(&u);
        println!("Success!\nu: {:?}", u);
    }
    pub fn exercise_003() {
        struct Color(i32, i32, i32);
        #[derive(Debug)]
        struct Point(i32, i32, i32);
        fn check_color(p: &Point) {
            let Point(x, _, z) = *p;
            assert_eq!(x, 0);
            assert_eq!(p.1, 127);
            assert_eq!(z, 255)
        }

        let v = Point(0, 127, 255);
        check_color(&v);
        println!("Success!\nv: {:?}", v);
    }
    pub fn exercise_004() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }
        let age = 18;
        let mut p = Person {
            name: String::from("sunface"),
            age
        };
        assert_eq!(p.age, 18);
        p.name = String::from("sunfei");
        println!("Success!\np: {:?}", p);
    }
    pub fn exercise_005() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }
        println!("Success!\nbuild_person returns {:?}", build_person("sunface".to_string(), 30));

        fn build_person(name:String, age: u8) -> Person {
            Person { 
                age, 
                name, 
            }
        }
    }
    pub fn exercise_006() {
        #[derive(Debug, Clone)]
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
        }
        let u1 = User {
            email: String::from("someone@example.com"),
            username: String::from("sunface"),
            active: true,
            sign_in_count: 1,
        };
        let u2 = set_email(u1.clone());
        println!("Success!\nu1: {:?}\nu2: {:?}", u1, u2);

        fn set_email(u: User) -> User {
            User {
                email: String::from("contact@im.dev"),
                ..u
            }
        }
    }
    pub fn exercise_007() {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        let scale = 2;
        let rect1 = Rectangle {
            // dbg!() Print debug info to stderr and assign the value.
            width: dbg!(30 * scale),
            height: 50,
        };
        dbg!(&rect1);
        println!("rect1: {:?}", rect1);
    }
    pub fn exercise_008() {
        #[derive(Debug)]
        struct File {
            name: String,
            data: String,
        }
        let f = File {
            name: String::from("readme.md"),
            data: "Rust By Practice".to_string()
        };
        let _name = f.name;
        println!("name: {}, data: {}", _name, f.data);
    }
}
