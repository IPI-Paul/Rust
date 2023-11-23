#![allow(unused)]
pub struct Traits;
impl Traits {
    pub fn exercise_001() {
        trait Hello {
            fn say_hi(&self) -> String {
                String::from("hi")
            }
            fn say_something(&self) -> String;
        }
        #[derive(Debug)]
        struct Student;
        impl Hello for Student {
            fn say_something(&self) -> String {
                String::from("I'm a good student")
            }
        }
        #[derive(Debug)]
        struct Teacher;
        impl Hello for Teacher {
            fn say_hi(&self) -> String {
                String::from("Hi, I'm your new teacher")
            }
            fn say_something(&self) -> String {
                String::from("I'm not a bad teacher")
            }
        }
        let s = Student {};
        assert_eq!(s.say_hi(), "hi");
        assert_eq!(s.say_something(), "I'm a good student");
        let t = Teacher {};
        assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
        assert_eq!(t.say_something(), "I'm not a bad teacher");
        println!(
            "Success!\ns: {:?}\ns.say_hi(): {}\ns.say_something(): {}\nt: {:?}\nt.say_hi(): {}\
            \nt.say_something(): {}",
            s, s.say_hi(), s.say_something(), t, t.say_hi(), t.say_something()
        );
    }
    pub fn exercise_002() {
        #[derive(PartialEq, PartialOrd)]
        struct Centimeters(f64);

        #[derive(Debug)]
        struct Inches(i32);
        impl Inches {
            fn to_centimeters(&self) -> Centimeters {
                let &Inches(inches) = self;
                Centimeters(inches as f64 * 2.54)
            }
        }

        #[derive(Debug, PartialEq, PartialOrd)]
        struct Seconds(i32);

        let _one_second = Seconds(1);
        println!("One second looks like: {:?}", _one_second);
        let _this_is_true = _one_second == _one_second;
        let _this_is_false = _one_second > _one_second;

        let foot = Inches(12);
        println!("One foot equals {:?}", foot);
        let meter = Centimeters(100.0);
        let cmp = if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };
        println!("One foot is {} than one meter.", cmp);
    }
    pub fn exercise_003() {
        use std::ops;
        fn multiply<T: ops::Mul<Output = T>>(a: T, b: T) -> T {
            a * b
        }
        assert_eq!(6, multiply(2u8, 3u8));
        assert_eq!(5.0, multiply(1.0, 5.0));
        println!("Success!\n2 * 3 = {}\n1.0 * 5.0 = {:.1}", multiply(2u8, 3u8), multiply(1.0, 5.0));
    }
    pub fn exercise_004() {
        use std::ops;
        struct Foo;
        struct Bar;
        #[derive(Debug, PartialEq)]
        struct FooBar;
        #[derive(Debug, PartialEq)]
        struct BarFoo;
        impl ops::Add<Bar> for Foo {
            type Output = FooBar;
            fn add(self, _rhs: Bar) -> FooBar {
                FooBar
            }
        }
        impl ops::Sub<Bar> for Foo {
            type Output = BarFoo;
            fn sub(self, _rhs: Bar) -> BarFoo {
                BarFoo
            }
        }
        assert_eq!(Foo + Bar, FooBar);
        assert_eq!(Foo - Bar, BarFoo);
        println!("Success!\nFoo + Bar = {:?}\nFoo - Bar = {:?}", Foo + Bar, Foo - Bar);
    }
    pub fn exercise_005() {
        trait Summary {
            fn summarize(&self) -> String;
        }
        #[derive(Debug)]
        struct Post {
            title: String,
            author: String,
            content: String,
        }
        impl Summary for Post {
            fn summarize(&self) -> String {
                format!("The author of post {} is {}", self.title, self.author)
            }
        }
        #[derive(Debug)]
        struct Weibo {
            username: String,
            content: String,
        }
        impl Summary for Weibo {
            fn summarize(&self) -> String {
                format!("{} published a weibo {}", self.username, self.content)
            }
        }
        let post = Post {
            title: "Popular Rust".to_string(),
            author: "Sunface".to_string(),
            content: "Rust is awesome".to_string(),
        };
        let weibo = Weibo {
            username: "sunface".to_string(),
            content: "Weibo seems to be worse than Tweet".to_string(),
        };

        summary(&post);
        summary(&weibo);

        println!("{:?}", post);
        println!("{:?}", weibo);
        println!("post: {}\nweibo: {}", summary_my(&post), summary_my(&weibo));

        fn summary_my<T: Summary>(x: &T) -> String {
            x.summarize()
        }

        fn summary(a: &impl Summary) {
            let output: String = a.summarize();
            println!("{}", output);
        }
    }
    pub fn exercise_006() {
        struct Sheep;
        struct Cow;
        trait Animal {
            fn noise(&self) -> String;
        }
        impl Animal for Sheep {
            fn noise(&self) -> String {
                "baaaaah!".to_string()
            }
        }
        impl Animal for Cow {
            fn noise(&self) -> String {
                "mooooooo!".to_string()
            }
        }
        fn random_animal_my(random_number: f64) -> &'static dyn Animal {
            if random_number < 0.5 {
                &Sheep {}
            } else {
                &Cow {}
            }
        }
        fn random_animal(random_number: f64) -> Box<dyn Animal> {
            if random_number < 0.5 {
                Box::new(Sheep {})
            } else {
                Box::new(Cow {})
            }
        }

        let random_number = 0.234;
        let animal = random_animal(random_number);
        println!(
            "You've randomly chosen an animal, and it says {} {}", 
            animal.noise(), random_animal_my(random_number).noise()
        );
    }
    pub fn exercise_007() {
        assert_eq!(sum(1, 2), 3);
        println!("sum(5, 5) = {}\nsum(5.0, 5.0) = {:.1}", sum(5, 5), sum(5.0, 5.0));

        fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
            x + y
        }
    }
    pub fn exercise_008() {
        struct Pair<T> {
            x: T,
            y: T,
        }
        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { 
                    x, 
                    y,
                }
            }
        }
        impl<T: std::fmt::Debug + PartialOrd + PartialEq> Pair<T> {
            fn cmp_display(&self) {
                if self.x >= self.y {
                    println!("The largest member is x = {:?}", self.x);
                } else {
                    println!("The largest member is y = {:?}", self.y);
                }
            }
        }
        #[derive(Debug, PartialEq, PartialOrd)]
        struct Unit(i32);

        let pair = Pair {
            x: Unit(1),
            y: Unit(3)
        };
        pair.cmp_display();
        let pair = Pair::new(Unit(1), Unit(3));
        pair.cmp_display();
    }
}
