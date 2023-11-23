#![allow(unused, non_upper_case_globals)]
pub struct StaticLifetimes;
impl StaticLifetimes {
    pub fn exercise_001() {
        let v = "hello";
        need_static(v);
        // Or
        let v: &'static str = "hello";
        need_static(v);
        // Or
        let v: &str = "hello";
        need_static(v);
        println!("Success!\nv: '{}'", v);

        fn need_static(r: &'static str) {
            assert_eq!(r, "hello");
        }
    }
    pub fn exercise_002() {
        // Using Box::leak to make 'static lifetimes is unsafe
        #[derive(Debug)]
        struct Config {
            a: String,
            b: String,
        }
        static mut config: Option<&mut Config> = None;
        fn init() -> Option<&'static mut Config> {
            Some(Box::leak(Box::new(Config {
                a: "A".to_string(),
                b: "B".to_string(),
            })))
        }
        unsafe {
            config = init();
            println!("{:?}", config)
        }
    }
    pub fn exercise_003() {
        {
            let static_string: &'static str = "I'm in read-only memory";
            println!("static_string: {}", static_string);
        }
        println!("static_string reference remains alive, but the variable is now out of scope!");
    }
    pub fn example_001() {
        // static will stay at same memory location when compiled, but const will be inlined to 
        // where it is referenced.
        static NUM: i32 = 18;
        // Returns a reference to NUM where its static lifetime is coerced to that of the input arg.
        fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
            &NUM
        }
        {
            let lifeime_num = 9;
            let coerced_static = coerce_static(&lifeime_num);
            println!("coerced_static: {}", coerced_static);
        }
        println!("NUM: {} stays accessible!", NUM);
    }
    pub fn exercise_005() {
        use std::fmt::Debug;
        fn print_it<T: Debug + 'static>(input: T) {
            println!("'static value passed in is: {:?}", input);
        }
        fn print_it1(input: impl Debug + 'static) {
            println!("'static value passed in is: {:?}", input);
        }
        fn print_it2<T: Debug + 'static>(input: &T) {
            println!("'static value passed in is: {:?}", input);
        }
        let i = 5;
        print_it(i);
        /* &i only has the lifetime defined by the scope and it's not static.
        To fix change i to a static or const.
        print_it(&i);
        print_it1(&i);
         */
        print_it2(&i);
    }
    pub fn exercise_006() {
        use std::fmt::Display;
        let mut string = "First".to_owned();
        string.push_str(string.to_uppercase().as_str());
        print_a(&string);
        print_b(&string);
        print_c(&string);
        print_d(&string);
        print_e(&string);
        print_f(&string);
        print_g(&string);

        fn print_a<T: Display + 'static>(t: &T) {
            println!("{}", t);
        }
        fn print_b<T>(t: &T)
        where
            T: Display + 'static,
        {
            println!("{}", t);
        }
        fn print_c<'a>(t: &'a dyn Display) {
            println!("{}", t);
        }
        fn print_d<'a>(t: &'a impl Display) {
            println!("{}", t);
        }
        fn print_e(t: &(dyn Display + 'static)) {
            println!("{}", t);
        }
        fn print_f(t: &(impl Display + 'static)) {
            println!("{}", t);
        }
        fn print_g<'a>(t: &'a String) {
            println!("{}", t);
        }
    }
}
