#![allow(unused)]
pub struct AssociatedTypes;
impl AssociatedTypes {
    pub fn exercise_001() {
        trait Bird {
            fn quack(&self) -> String;
        }
        struct Duck;
        impl Duck {
            fn swim(&self) {
                println!("Look, the duck is swimming")
            }
        }
        struct Swan;
        impl Swan {
            fn fly(&self) {
                println!("Look, the duck.. oh sorry, the swan is flying")
            }
        }
        impl Bird for Duck {
            fn quack(&self) -> String {
                "duck duck".to_string()
            }
        }
        impl Bird for Swan {
            fn quack(&self) -> String {
                "swan swan".to_string()
            }
        }

        let duck = Duck;
        duck.swim();

        let bird = hatch_a_bird(2);
        // bird.swim();
        assert_eq!(bird.quack(), "duck duck");

        let bird = hatch_a_bird(1);
        // bird.fly();
        assert_eq!(bird.quack(), "swan swan");

        println!("Success!");
        fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
            match species {
                1 => Box::new(Swan),
                2 => Box::new(Duck),
                _ => panic!(),
            }
        }
    }
    pub fn exercise_002() {
        trait Bird {
            fn quack(&self);
        }
        struct  Duck;
        impl Duck {
            fn fly(&self) {
                println!("Look, the duck is flying")
            }
        }
        struct Swan;
        impl Swan {
            fn fly(&self) {
                println!("Look, the duck.. oh sorry, the swan is flying")
            }
        }
        impl Bird for Duck {
            fn quack(&self) {
                println!("{}", "duck duck")
            }
        }
        impl Bird for Swan {
            fn quack(&self) {
                println!("{}", "swan swan")
            }
        }
        // let birds: [Box<dyn Bird>; 2] = [Box::new(Duck), Box::new(Swan)];
        // Or
        let birds: [&dyn Bird; 2] = [&Duck, &Swan];
        for bird in birds {
            bird.quack();
            // bird.fly() not a trait function.
        }
    }
    pub fn exercise_003() {
        trait Draw {
            fn draw(&self) -> String;
        }
        impl Draw for u8 {
            fn draw(&self) -> String {
                format!("u8: {}", self)
            }
        }
        impl Draw for f64 {
            fn draw(&self) -> String {
                format!("f64: {}", self)
            }
        }
        let x = 1.1f64;
        let y = 8u8;
        draw_with_box(Box::new(x));
        draw_with_ref(&y);
        println!(
            "Success!\ndraw_with_box(Box::new(x)): {}\ndraw_with_ref(&y): {}",
            draw_with_box(Box::new(x)), draw_with_ref(&y)
        );

        fn draw_with_box(x: Box<dyn Draw>) -> String {
            x.draw()
        }
        fn draw_with_ref(x: &dyn Draw) -> String {
            x.draw()
        }
    }
    pub fn exercise_004() {
        trait Foo {
            fn method(&self) -> String;
        }
        impl Foo for u8 {
            fn method(&self) -> String {
                format!("u8: {}", self)
            }
        }
        impl Foo for String {
            fn method(&self) -> String {
                format!("string: {}", self)
            }
        }
        fn static_dispatch<T: Foo>(x: T) -> String {
            x.method()
        }
        fn dynamic_despatch(x: &dyn Foo) -> String {
            x.method()
        }
        let x = 5u8;
        let y = "Hello".to_string();
        static_dispatch(x);
        dynamic_despatch(&y);
        println!(
            "Success!\nstatic_dispatch(x): {}\ndynamic_despatch(&y): {}",
            static_dispatch(x), dynamic_despatch(&y)
        )
    }
    pub fn exercise_005() {
        trait MyTrait {
            fn f(&self) -> Self;
        }
        impl MyTrait for u32 {
            fn f(&self) -> Self {
                42
            }
        }
        impl MyTrait for String {
            fn f(&self) -> Self {
                self.clone()
            }
        }
        fn my_function<T: MyTrait>(x: T) -> T {
            x.f()
        }
        my_function(13_u32);
        my_function(String::from("abc"));
        println!(
            "Success!\nmy_function(13_u32): {}\nmy_function(String::from(\"abc\")): {}",
            my_function(13_u32), my_function(String::from("abc"))
        );
        trait MyTrait1: std::fmt::Debug {
            fn f(&self) -> Box<dyn MyTrait1>;
        }
        impl MyTrait1 for u32 {
            fn f(&self) -> Box<dyn MyTrait1> {
                Box::new(42)
            }
        }
        impl MyTrait1 for String {
            fn f(&self) -> Box<dyn MyTrait1> {
                Box::new(self.clone())
            }
        }
        fn my_function1(x: Box<dyn MyTrait1>) -> Box<dyn MyTrait1> {
            x.f()
        }
        my_function1(Box::new(13_u32));
        my_function1(Box::new(String::from("abc")));
        println!(
            "Success!\nmy_function1(Box::new(13_u32)): {:?}\nmy_function1(Box::new(String::from(\"abc\"))): {:?}",
            my_function1(Box::new(13_u32)), my_function1(Box::new(String::from("abc")))
        );
    }
}
