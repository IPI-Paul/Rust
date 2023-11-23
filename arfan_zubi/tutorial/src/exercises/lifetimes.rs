#![allow(unused)]
pub struct Lifetimes;
impl Lifetimes {
    pub fn exercise_001() {
        let i = 3;
        println!("i lifetime begins: {}", i);
        {
            let borrow1 = &i;
            println!("borrow1 lifetime begins: {}", borrow1);
        }
        println!("borrow1 lifetime ends");
        {
            let borrow2 = &i;
            println!("borrow2 lifetime begins: {}", borrow2);
        }
        println!("borrow2 lifetime ends");
        println!("i lifetime will end when function goes out of scope")
    }
    pub fn example_001() {
        let x = 5;
        println!("x lifetime begins: {}", x);
        let r = &x;
        println!("r lifetime begins: {}", r);
        println!("both x and r lifetimes will end when function goes out of scope");
    }
    pub fn exercise_002() {
        {
            let r;
            println!("r lifetime begins uninitialised");
            {
                let x = 5;
                println!("x lifetime begins: {}", x);
                r = &x;
                println!("r is initialised with a reference to x: {}", r);
            }
            println!("x lifetime ends");
            println!("r is now a dangling reference as x no longer exists:", )
        }
    }
    pub fn example_002() {
        // The lifetime specified as part of the function name will out
        // live the function. Lifetimes go from a to z and on.
        fn print_one<'a>(x: &'a i32) {
            println!("`print_one`: x is {}", x);
        }
        fn add_one<'a>(x: &'a mut i32) {
            *x += 1;
        }
        // Multiple elements can have different lifetimes.
        fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
            println!("`print_multi`: x is {}, y is {}", x, y);
        }
        fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
            x
        }
        let x = 7;
        let y = 9;
        print_one(&x);
        print_multi(&x, &y);
        let z = pass_x(&x, &y);
        print_one(&z);
        let mut t = 3;
        add_one(&mut t);
        print_one(&t);
    }
    pub fn exercise_003() {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        let x = "long";
        let y = "longer";
        println!("{}", longest(x, y));
    }
    pub fn exercise_004() {
        fn invalid_output() -> String {
            String::from("foo")
        }
        fn invalid_output1() -> &'static str {
            "foo"
        }
        fn invalid_output2<'a>(s: &'a str) -> &'a str {
            s
        }
        let x = invalid_output();
        let x1 = invalid_output1();
        let s = String::from("foo");
        let x2 = invalid_output2(&s);
        println!("x: {}\nx1: {}\nx2: {}", x, x1, x2);
    }
    pub fn exercise_005() {
        fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
            println!("x is {} and y is {}", x, y);
        }
        fn failed_borrow() {
            let _x = 12;
            let y: &i32 = &_x;
            print_refs(&_x, y);
        }
        let (four, nine) = (4, 9);
        print_refs(&four, &nine);
        failed_borrow();
    }
    pub fn exercise_006() {
        #[derive(Debug)]
        struct Borrowed<'a>(&'a i32);

        #[derive(Debug)]
        struct NamedBorrowed<'a> {
            x: &'a i32,
            y: &'a i32,
        }

        #[derive(Debug)]
        enum Either<'a> {
            Num(i32),
            Ref(&'a i32),
        }
        let x = 18;
        let y = 15;
        let single = Borrowed(&x);
        let double = NamedBorrowed {x: &x, y: &y};
        let reference = Either::Ref(&x);
        let number = Either::Num(y);
        println!("x is borrowed in {:?}", single);
        println!("x and y are borrowed in {:?}", double);
        println!("x is borrowed in {:?}", reference);
        println!("y is *not* borrowed in {:?}", number);
    }
    pub fn exercise_007() {
        #[derive(Debug)]
        struct NoCopyType {}

        #[derive(Debug)]
        struct Example<'a, 'b> {
            a: &'a u32,
            b: &'b NoCopyType,
        }
        let var_a = 35;
        let mut example = Example {a: &var_a, b: &NoCopyType {}};
        {
            let var_b = NoCopyType {};
            example = Example { a: &var_a, b: &var_b};
            println!("Success!\nexample: {:?}", example);
        }
    }
    pub fn exercise_008() {
        #[derive(Debug)]
        struct NoCopyType {}

        #[derive(Debug)]
        #[allow(dead_code)]
        struct Example<'a, 'b> {
            a: &'a u32,
            b: &'b NoCopyType,
        }

        fn fix_me<'a>(foo: &'a Example) -> &'a NoCopyType {
            foo.b
        }
        let no_copy = NoCopyType {};
        let example = Example { a: &1, b: &no_copy };
        fix_me(&example);
        println!("Success!\nexample: {:?}\nfix_me(&example): {:?}", example, fix_me(&example))
    }
    pub fn example_003() {
        struct Owner(i32);
        impl Owner {
            fn add_one<'a>(&'a mut self) { self.0 += 1; }
            fn print<'a>(&'a self) {
                println!("`print`: {}", self.0);
            }
        }
        let mut owner = Owner(18);
        owner.add_one();
        owner.print();
    }
    pub fn exercise_009() {
        struct ImportantExcerpt {
            part: &'static str,
        }
        impl ImportantExcerpt {
            fn level(&self) -> i32 {
                3
            }
        }
        let x = ImportantExcerpt {part: &""};
        println!("x: {}", x.level());
    }
    pub fn exercise_010() {
        fn input(x: &i32) {
            println!("`annotated_input`: {}", x);
        }
        fn pass(x: &i32) -> &i32 { x }
        fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
            x
        }
        struct Owner(i32);
        // Mutable/immutable self has the lifetime inferred.
        impl Owner {
            fn add_one(&mut self) {
                self.0 += 1
            }
            fn print(&self) {
                println!("`print`: {}", self.0);
            }
        }
        // Can use static lifetime instead of adding individual lifetime a.
        struct Person {
            age: u8,
            name: &'static str,
        }
        println!("Success!")
    }
}
