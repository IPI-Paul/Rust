pub struct Variables;
impl Variables {
    // Variables.
    // A variable can be used only if it has been initialised.
    pub fn exercise_001() {
        // Uninitialised but used, ERROR!
        let x: i32;
        // Uninitialised but also unused, only a Warning! To avoid warning 
        // prepend with underscore let _y: i32;
        let y: i32; 
        // initialise x.
        x = 5;
        assert_eq!(x, 5);
        println!("Success!\nx = {}", x);
    }
    // Use mut to mark a variable as mutable.
    pub fn exercise_002() {
        let mut x: i32 = 1;
        x += 2;
        assert_eq!(x, 3);
        println!("Success!\nx = {}", x);
    }
    // Scope.
    // A scope is the range within the program for which the item is valid.
    pub fn exercise_003() {
        // Outer scope.
        let x: i32 = 10;
        {
            // Inner scope.
            let y: i32 = 5;
            println!("The value of x is {} and the value of y is {}", x, y);
        }
        // Or Tutors example: define y in outer scope.
        // println!("The value of x is {} and the value of y is {}", x, y);
    }
    pub fn exercise_004() {
        fn main() {
            println!("{}, world", define_x());
        }
    
        fn define_x() -> String {
            let x = "hello".to_string();
            x
        }
        main()
        /* Or Tutors example:
        fn main() {
            define_x();
        }
        fn define_x() {
            let x: &str = "hello";
            println!()"{}, world", x);
        }
         */
    }
    // Shadowing
    pub fn exercise_005() {
        let x: i32 = 5;
        {
            let x = 12;
            assert_eq!(x, 12)
        }
        assert_eq!(x, 5);
        let x = 42;
        // Prints 42
        println!("{}", x);
    }
    pub fn exercise_006() {
        let mut x: i32 = 1;
        x = 7;
        // Shadowing and re-binding. 
        // let x = x;
        x += 3;
        
        let y = 4;
        // Shadowing.
        let y = "I can also be bound to text!";
        println!("Success!\n x = {}\n y = {}", x, y);
    }
    // Unused variables.
    pub fn exercise_007() {
        // To avoid warning, prepend with underscore
        let _x = 1;
        // Or, tell compiler to ignore.
        fn main() {
            let x = 1;
            println!("x: {}", x);
        }
        main();
        println!("_x: {}", _x);
    }
    // Destructuring.
    // Can use pattern with let to destructure a tuple to separate values.
    pub fn exercise_008() {
        let (mut x, y) = (1, 2);
        x += 2;
    
        assert_eq!(x, 3);
        assert_eq!(y, 2);
        println!("Success!\n x = {}\n y = {}", x, y);
    }
    // Destructuring assignments.
    // Rust version 1.59 onwards: use tuple, slice, and struct patterns as the left-hand side 
    // of an assignment.
    pub fn exercise_009() {
        let (x, y);
        (x, ..) = (3, 4);
        [.., y] = [1, 2];
        assert_eq!([x, y], [3, 2]);
        println!("Success!\n x = {}\n y = {}", x, y);
    }
}

pub struct Integers;
impl Integers {
    // arc (isize and usize) = Computers architechture.
    pub fn exercise_001() {
        let x: i32 = 5;
        let mut y = 5;
        y = x;
        /* Or:
        let mut y: u32 = 5;
        y = x as u32;
         */
        let z = 10;
        println!("Success!\nx = {}\ny = {}\nz = {}", x, y, z);
    }
    pub fn exercise_002() {
        let v: u16 = 38_u8 as u16;
        println!("Success!\nv as u16 = {} as u8 cast as u16", v);
    }
    // If a type is not explicitly assigned to a variable the compiler will infer one.
    pub fn exercise_003() {
        let x = 5;
        // Google searched.
        // fn type_of<T>(_: T) -> &'static str {
        //     type_name::<T>()
        // }
        /* Tutor changed annotation: 
            let x: u32 = 5;
            assert_eq!("u32".to_string(), type_of(&x));
        */
        assert_eq!("i32".to_string(), type_of(&x));
        println!("Success!\nType of x is {}",type_of(&x));
        // Get the type of a given variable and return a string representation of the type.
        fn type_of<T>(_: &T) -> String {
            format!("{}", std::any::type_name::<T>())
        }
    }
    pub fn exercise_004() {
        assert_eq!(i8::MAX, 127);
        assert_eq!(u8::MAX, 255);
        println!("Success!\ni8::MAX = {}\nu8::MAX = {}", i8::MAX, u8::MAX)
    }
    pub fn exercise_005() {
        let v1 = 251_u16 + 8;
        let v2 = i16::checked_add(251, 8).unwrap();
        println!("{}, {}", v1, v2);
    }
    pub fn exercise_006() {
        //          decimal  hexa decimal 255  octal 63      binary 255.
        let v = 1_024 +        0xff +       0o77 +    0b1111_1111;
        assert!(v == 1597);
        println!("Success!\nv = {}", v);
    }
}

pub struct FloatingPoint;
impl FloatingPoint {
    pub fn exercise_001() {
        let x = 1_000.000_1;
        let y: f32 = 0.12;
        let z = 0.01_f64;

        assert_eq!(type_of(&x), "f64".to_string());
        println!(
            "Success!\nType of x is {} and = {}\nType of y is {} and = {}\nType of z is {} and = {}",
            type_of(&x), x, type_of(&y), y, type_of(&z), z
        );
        // Get the type of a given variable and return a string representation of the type.
        fn type_of<T>(_: &T) -> String {
            format!("{}", std::any::type_name::<T>())
        }
    }
    pub fn exercise_002() {
        // Panic caused by floating point precision. Implicitly uses f64.
        // assert!(0.1 + 0.2 == 0.3);
        assert!(0.1_f32 + 0.2 == 0.3);
        // Or.
        assert!(0.1 as f32 + 0.2 == 0.3);
        println!("Success!\n{} + {} = {}", 0.1, 0.2, 0.3);
    }
    pub fn exercise_003() {
        let mut sum = 0;
        for i in -3..2 {
            println!("{} + {} = {}", sum, i, sum + i);
            sum += i
        }
        println!("");
        assert!(sum == -5);
        for c in 'a'..='z' {
            print!("{} ", c);
        }
        println!("");
        for c in 'a'..='z' {
            print!("{} ", c as u8);
        }
    }
    pub fn exercise_004() {
        use std::ops::{Range, RangeInclusive};
        assert_eq!((1..5), Range{ start: 1, end: 5 });
        assert_eq!((1..=5), RangeInclusive::new(1, 5));
        println!("Success!")
    }
}

pub struct Computations;
impl Computations {
    pub fn exercise_001() {
        // Integer addition.
        assert!(1u32 + 2 == 3);
        // Integer subtraction.
        assert!(1i32 - 2 == -1);
        assert!(1i8 - 2 == -1);
        assert!(3 * 50 == 150);
        assert!(9.6f32 / 3.2 == 3.0);
        assert!(24 % 5 == 4);
        // Short circuiting boolean logic.
        assert!(true && false == false);
        assert!(true || false == true);
        assert!(!true == false);
        // Bitwise operations.
        println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
        println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
        println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
        println!("1 << 5 is {} and in binary {1:04b} shifted 5 places is {0:04b}", 1u32 << 5, 1u32);
        println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    }
}

pub struct CharBoolUnit;
impl CharBoolUnit {
    pub fn exercise_001() {
        use std::mem::size_of_val;
        let c1 = 'a';
        assert_eq!(size_of_val(&c1), 4);
        let c2 = '中';
        assert_eq!(size_of_val(&c2), 4);
        println!(
            "Success!\nCaracter size of {} = {}\nCharacter size of {} = {}",
            c1, size_of_val(&c1), c2, size_of_val(&c2)
        );
    }
    pub fn exercise_002() {
        let c1 = '中';
        print_char(c1);

        fn print_char(c: char) {
            println!("{}", c);
        }
    }
    pub fn exercise_003() {
        let _f: bool = false;
        let t = true;
        if t {
            println!("Success!\nt is {}", t);
        }
    }
    pub fn exercise_004() {
        let f = true;
        let t = true || false;
        assert_eq!(t, f);
        println!("Success!\ntrue || false = {}", t);
    }
    pub fn exercise_005() {
        let _v: () = ();
        let v = (2, 3);
        assert_eq!(_v, implicitly_ret_unit());
        println!("Success!\nv = {:?}\n_v = {:?}", v, _v);

        fn implicitly_ret_unit() {
            println!("I will return a ()");
        }
        // Don't use this one
        fn explicitly_ret_unit() -> () {
            println!("I will return a ()");
        }
    }
    pub fn exercise_006() {
        use std::mem::size_of_val;
        let unit: () = ();
        assert!(size_of_val(&unit) == 0);
        println!("Success!\nThe size of unit = {}", size_of_val(&unit));
    }
}

pub struct ExpressionStatement;
impl ExpressionStatement {
    pub fn exercise_001() {
        let x = 5u32;
        let y = {
            let x_squared = x * x;
            let x_cube = x_squared * x;
            // This expression will be assigned to y.
            x_cube + x_squared + x
        };
        let z = {
            // The semicolon suppresses this expression and () is assigned to z.
            2 * x;
        };
        println!("x is {:?}", x);
        println!("y is {:?}", y);
        println!("z is {:?}", z);
    }
    pub fn exercise_002() {
        let v = {
            let mut x = 1;
            x += 2
        };
        assert_eq!(v, ());
        println!("Success!\nv = {:?}", v);
        let v = {
            let mut x = 1;
            x += 2;
            x
        };
        assert_eq!(v, 3);
        println!("v = {}", v);
    }
    pub fn exercise_003() {
        // let v = (let x = 3);
        let v = {
            let x = 3; 
            x
        };
        assert!(v == 3);
        println!("Success!\nv = {}", v);
    }
    pub fn exercise_004() {
        let s = sum(1, 2);
        assert_eq!(s, 3);
        println!("Success!\ns = {}", s);
        
        fn sum(x: i32, y: i32) -> i32 {
            x + y
        }
    }
}

pub struct Functions;
impl Functions {
    pub fn exercise_001() {
        let (x, y) = (1, 2);
        let s = sum(x, y);
        assert_eq!(s, 3);
        println!("Success!\nsum(x, y) = {:?}", sum(x, y));
        
        fn sum(x: i32, y: i32) -> i32 {
            x + y
        }
    }
    pub fn exercise_002() {
        print();

        fn print() {
            println!("Success!")
        }
    }
    pub fn exercise_003() {
        never_return();
        println!("Failed!");

        // Diverging function never return to the caller and may be used in places where a value of 
        // any type is expected.
        fn never_return() -> ! {
            // Implement this function, don't modify the fn signatures.
            panic!("Stop programme!");
        }
    }
    pub fn exercise_004() {
        println!("Success!");

        fn get_option(tp: u8) -> Option<i32> {
            match tp {
                1 => {
                    todo!()
                },
                _ => {
                    todo!()
                }
            };
            never_return()
        }
        fn never_return() -> ! {
            panic!("Stop programme!");
            unimplemented!();
            todo!()
        }
    }
    pub fn exercise_005() {
        let b = false;
        let v = match b {
            true => 1,
            // Diverging functions can also be used in match expressions to replace a value.
            false => {
                println!("Success!");
                panic!("We have no value for false, but we can panic");
            }
        };
        println!("Exercise Failed if printing oput this line!");
    }
}

pub struct Ownership;
impl Ownership {
    pub fn exercise_001() {
        let s = String::from("hello");
        takes_ownership(s);
        let x = 5;
        makes_copy(x);
        println!(
            "s pointer to string was moved and went out of scope!\nx value was copied and = {}", 
            x
        );

        fn takes_ownership(some_string: String) {
            println!("{}", some_string);
        }
        fn makes_copy(some_integer: i32) {
            println!("{}", some_integer);
        }
    }
    pub fn exercise_002() {
        let s1 = gives_ownership();
        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);
        println!(
            "s receives ownership of pointer to string value and = {0} \
            \ns2 pointer to the string value was moved to s3 and s3 = {1}", 
            s1, s3
        );

        fn gives_ownership() -> String {
            let some_string = String::from("yours");
            some_string
        }
        fn takes_and_gives_back(a_string: String) -> String {
            a_string
        }
    }
    pub fn exercise_003() {
        let x = String::from("hello, world");
        let y = &x;
        println!("x = {}, y = {}", x, y);
        let y = x.clone();
        println!("x = {}, y = {}", x, y);
        let y = x.to_string();
        println!("x = {}, y = {}", x, y);
        let y = x.chars().into_iter().collect::<String>();
        println!("x = {}, y = {}", x, y);
    }
    pub fn exercise_004() {
        let s1 = String::from("hello, world");
        let s2 = take_ownership(s1);
        println!("s1 pointer to string has been moved to s2 and s2 = {}", s2);

        fn take_ownership(some_string: String) -> String {
            println!("pointer to string '{}' has been moved to function.", some_string);
            some_string
        }
    }
    pub fn exercise_005() {
        let s = give_ownership();
        println!("s receives ownership of pointer to '{}'", s);

        fn give_ownership() -> String {
            let s = String::from("hello, world");
            // let _s = s.clone().into_bytes();
            let _s = s.as_bytes();
            println!("_s = {:?}", _s);
            s
        }
    }
    pub fn exercise_006() {
        let s = String::from("hello, world");
        // print_str(s.to_string());
        print_str(s.clone());
        println!("{}", s);

        fn print_str(s: String) {
            println!("{}", s)
        }
    }
    pub fn exercise_007() {
        /* Don't use clone:
        let x = (1, 2, (), "hello".to_string());
        let y = x.clone();
        let y = (x.0, x.1, x.2, &x.3);
        let y = (x.0, x.1, x.2, x.3.to_string());
        &str is immutable and is of known size. So can be copied.
        */
        let x = (1, 2, (), "hello");
        let y = x;
        println!("x = {:?}\ny = {:?}", x, y);
    }
    pub fn exercise_008() {
        let s = String::from("hello, ");
        let mut s1 = s;
        s1.push_str("world");
        println!("Success!\ns1 = {}", s1);
    }
    pub fn exercise_009() {
        let x = Box::new(5);
        let mut y = Box::new(0);
        *y = 4;
        assert_eq!(*x, 5);
        println!("Success!\nx = {}\ny = {}", x, y);
    }
}

pub struct PartialMove;
impl PartialMove {
    pub fn exercise_001() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: Box<u8>,
        }
        let person = Person {
            name: String::from("Alice"),
            age: Box::new(20),
        };
        // name is moved out of person but 'age' is referenced.
        // Destructuring into variables. ref is a keyword for reference/pointer/borrow.
        let Person { name, ref age } = person;
        println!("The person's age is {}", age);
        println!("The person's name is {}", name);
        // person instance cannot be used as it no longer owns name
        println!("The person instance age = {}", person.age);
    }
    pub fn exercise_002() {
        let t = (String::from("hello"), String::from("world"));
        let _s = t.0;
        println!("Partial move occured on t to _s and t now = {:?} and _s = {}", t.1, _s);
    }
    pub fn exercise_003() {
        let t = (String::from("hello"), String::from("world"));
        let (s1, s2) = t.clone();
        // let (s1, s2) = &t;
        println!("s1: {:?}, s2: {:?}, t: {:?}", s1, s2, t);
    }
}

pub struct ReferenceBorrowing;
impl ReferenceBorrowing {
    pub fn exercise_001() {
        let x = 5;
        let p = &x;
        // {:p} returns the memory address of a pointer
        println!("the memory address of x is {:p}", p)
    }
    pub fn exercise_002() {
        let x = 5;
        let y = &x;
        assert_eq!(5, *y);
        print!("Success!\nx = {}\ny = {}", x, y);
    }
    pub fn exercise_003() {
        let mut s = String::from("hello, ");
        borrow_object(&s);
        println!("Success!\ns: '{}' was borrowed.", s);

        fn borrow_object(s: &String) {}
    }
    pub fn exercise_004() {
        let mut s = String::from("hello, ");
        push_str(&mut s);
        println!("Success!\ns was updated to '{}'", s);

        fn push_str(s: &mut String) {
            s.push_str("world")
        }
    }
    pub fn exercise_005() {
        let mut s = String::from("hello, ");
        // let mut p = s.to_string();
        let p = &mut s;
        p.push_str("world");
        println!("Success!\ns cannot be used again as it has been borrowed as mutable. \
            \np borowed s as mutable and was updated to '{}'", p
        );
    }
    pub fn exercise_006() {
        let c = '中';
        let r1 = &c;
        let ref r2 = c;
        assert_eq!(*r1, *r2);
        assert_eq!(get_addr(r1), get_addr(r2));
        println!("Success!\nr1 at {} = '{}'\nr2 at {} = '{}'", get_addr(r1), r1, get_addr(r2), r2);

        // Get memory address of string
        fn get_addr(r: &char) -> String {
            format!("{:p}", r)
        }
    }
    pub fn exercise_007() {
        let mut s = String:: from("hello");
        let r1 = &s;
        let r2 = &s;
        println!("r1: '{}'\nr2: '{}'\ns: '{}'", r1, r2, s);
        println!("Success!");
    }
    pub fn exercise_008() {
        let mut s = String::from("hello, ");
        borrow_object(&mut s);
        println!("Success!\ns: '{}'", s);

        fn borrow_object(s: &mut String) {}
    }
    pub fn exercise_009() {
        let mut s = String::from("hello, ");
        borrow_object(&s);
        s.push_str("world");
        println!("Success!\ns updated to '{}'", s);
        
        fn borrow_object(s: &String) {}
    }
    pub fn exercise_010() {
        let mut s = String::from("hello, ");
        let r1 = &mut s;
        r1.push_str("world");
        let r2 = &mut s;
        r2.push_str("!");
        /* Comment out to compile as r1 is nolonger being used.
        println!("r1: '{}'", r1);
        Or
        Don't use r1 so that there is not more than one mutable reference in scope.
         */
        println!("r2: '{}'", r2);
    }
    pub fn exercise_011() {
        let mut s = String::from("hello, ");
        let r1 = &mut s;
    //     let r2 = &mut s;
    //     // r1.push_str("world");
    //     println!("{}, {}", r1, r2)
    }
}

pub struct CompoundTypes;
impl CompoundTypes {
    // String is a compound type because it is made up of char types.
    pub fn exercise_001() {
        let s: &str = "hello, world";
        println!("Success!\ns: '{}'", s);
    }
    pub fn exercise_002() {
        let s: Box<str> = "hello, world".into();
        greetings(&s);
        let s: &str = "hello, world";
        greetings(s);

        fn greetings(s: &str) {
            println!("s: '{}'", s)
        }
    }
    pub fn exercise_003() {
        let mut s = String::new();
        s.push_str("hello, world");
        s.push('!');
        assert_eq!(s, "hello, world!");
        println!("Success!\ns: '{}'", s);
    }
    pub fn exercise_004() {
        let mut s = String::from("hello");
        s.push(',');
        s.push_str(" world");
        // s += &"!".to_string();
        s += "!";
        println!("s: '{}'", s);
    }
    pub fn exercise_005() {
        let s = String::from("I like dogs");
        //  Allocae new memory and store the modified string there.
        let s1 = s.replace("dogs", "cats");
        assert_eq!(s1, "I like cats");
        println!("Success!\ns: '{}'\ns1: '{}'", s, s1);
    }
    pub fn exercise_006() {
        let s1 = String::from("hello,");
        let s2 = String::from("world!");
        let s3 = s1.clone() + &s2;
        // let s3 = s1.clone() + s2.as_str();
        assert_eq!(s3, "hello,world!");
        println!("s1: '{}'\ns2: '{}'\ns3: '{}'", s1, s2, s3);
    }
    pub fn exercise_007() {
        let s = "hello, world";
        greetings(s.to_string());
        greetings(String::from(s));
        greetings(s.to_owned());

        fn greetings(s: String) {
            println!("s: '{}'", s)
        }
    }
    pub fn exercise_008() {
        let s = "hello, world".to_string();
        let s1: &str = &s;
        println!("Success!\ns: '{}'\ns1: '{}'", s, s1);
        let s1: &str = s.as_str();
        println!("s: '{}'\ns1: '{}'", s, s1);
    }
    pub fn exercise_009() {
        // You can use escapes to writ bytes by their hexadecimal values;
        let byte_escape = "I'm writing Ru\x73\x74!";
        println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape) ;
        // Or Unicode points.
        let unicode_codepoint = "\u{211D}";
        let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
        println!("Unicode character {} (U+211D) is called {}", unicode_codepoint, character_name);
        let long_string = "String literals 
                                can span multiple lines. 
                                The linebreak and indentation here \
                                 can be escaped too!";
        println!("{}", long_string);
    }
    pub fn exercise_010() {
        // let raw_str = r"Escapes don't work here: \x3F \u{211D}";
        let raw_str = "Escapes don't work here: \x3F \u{211D}";
        assert_eq!(raw_str, "Escapes don't work here: ? ℝ");
        let quotes = r#"And then I said: "There is no escape!""#;
        println!("quotes: {}", quotes);
        // You can use up to 65535 #s.
        let delimiter = r###"A string with "# in it. And even "##!"###;
        println!("delimiter: {}", delimiter);
        let long_delimiter= r###"Hello, "##""###;
        assert_eq!(long_delimiter, "Hello, \"##\"");
        println!("long_delimiter: {}", long_delimiter);
    }
    pub fn exercise_011() {
        // You can't use index to access a char in a string, but you can in a slice.
        // 中国 chinese zhōng guó for middle country.
        let s1 = String::from("hi,中国");
        let h = &s1[0..1];
        assert_eq!(h, "h");
        let h1 = &s1[3..6];
        assert_eq!(h1, "中");
        println!("Success!\ns1: '{}' meaning hi,middle country (zhōng guó)\nh: '{}'\nh1: '{}'", s1, h, h1);
    }
    pub fn exercise_012() {
        // 你好, 世界 chinese Nǐ hăo, shìjiè for Hello, world.
        for c in "你好, 世界".chars() {
            print!("{}", c)
        }
        println!("\n你好, 世界 is chinese Nǐ hăo, shìjiè for Hello, world.");
    }
}

pub struct Arrays;
impl Arrays {
    // Cannot be dynamical generated as the compiler needs to know the size at compile time.
    pub fn exercise_001() {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        assert!(arr.len() == 5);
        println!("Success!\narr: {:?}", arr);
    }
    pub fn exercise_002() {
        // Let compiler infer both type and size.
        let arr0 = [1, 2, 3];
        // Set the size but leave the compiler to infer the type.
        let arr: [_; 3] = ['a', 'b', 'c'];
        assert!(std::mem::size_of_val(&arr) == 12);
        println!(
            "Success!\narr0: {:?}\narr: {:?} is {} bytes", arr0, arr, std::mem::size_of_val(&arr)
        );
    }
    pub fn exercise_003() {
        let list: [i32; 100] = [1; 100];
        assert!(list[0] == 1);
        assert!(list.len() == 100);
        println!("Success!\nlist has {} {}s", list.len(), list[0]);
    }
    pub fn exercise_004() {
        let _arr = [1, 2, 3];
        println!("Success!\n_arr: {:?}", _arr);
    }
    pub fn exercise_005() {
        let arr = ['a', 'b', 'c'];
        let ele = arr[0];
        assert!(ele == 'a');
        println!("Success!\narr: {:?}\nele: {}", arr, ele);
    }
    pub fn exercise_006() {
        let names = [String::from("Sunfei"), "Sunface".to_string()];
        // Get returns an Option<T> an is safe to use.
        let name0 = names.get(0).unwrap();
        // Indexing is not safe.
        // let name1 = &names[2];
        let name1 = names.get(2);
        println!("Success!\nnames: {:?}\nname0: {}\n_name1: {:?}", names, name0, name1);
    }
}

pub struct Slices;
impl Slices {
    // You cannot use slices directly as their size is not known at compile time.
    // You have to use a pointer to them.
    pub fn exercise_001() {
        let arr = [1, 2, 3];
        let s1: &[i32] = &arr[0..2];
        // let s2: &str = "hello, world" as &str;
        let s2: &str = "hello, world";
        println!("Success!\narr: {:?}\ns1: {:?}\ns2: '{}'", arr, s1, s2);
    }
    pub fn exercise_002() {
        // [中 国 人] 类 Chinese for [Middle Country Human] Race.
        let arr : [char; 3] = ['中', '国', '人'];
        let slice = &arr[..2];
        // A slice reference is a two word object with a pointer and a length both as usize types. 
        // usize is 8 bytes on a 64 bit os.
        assert!(std::mem::size_of_val(&slice) == 16);
        println!(
            "Success!\narr: {:?} is chinese for ['Middle', 'Country','Human'] \
            \nslice: {:?} with a size of {}", 
            arr, slice, std::mem::size_of_val(&slice)
        )
    }
    pub fn exercise_003() {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let slice: &[i32] = &arr[1..4];
        assert_eq!(slice, &[2, 3, 4]);
        println!("Success!\narr: {:?}\nslice: {:?}", arr, slice);
    }
    pub fn exercise_004() {
        let s = String::from("hello");
        let slice1 = &s[0..2];
        let slice2 = &s[..2];
        assert_eq!(slice1, slice2);
        println!("Success!\ns: '{}'\nslice1: '{}'\nslice2: '{}'", s ,slice1, slice2);
    }
    pub fn exercise_005() {
        let s = "你好, 世界";
        let slice = &s[0..4];
        assert!(slice == "你");
        println!(
            "Success\ns: '{}' is chinese Nǐ hăo, shìjiè for Hello, world.\
            \nslice :'{}' is chinese Nǐ for Hello (hăo is you).",
            s, slice
        )
    }
    pub fn exercise_006() {
        let mut s = String::from("hello world");
        let word = first_word(&s);
        println!("the first word is: {}", word);
        s.clear();
        // println!("the first word is: {}", word);
        
        fn first_word(s: &str) -> &str {
            &s[..1]
        }
    }
}

pub struct Tuples;
impl Tuples {
    pub fn exercise_001() {
        let _t0: (u8, i16) = (0, -1);
        let _t1: (u8, (i16, u32)) = (0, (-1, 1));
        let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
        println!("Success!\n_t0: {:?}\n_t1: {:?}\nt: {:?}", _t0, _t1, t);
    }
    pub fn exercise_002() {
        let t = ("i", "am", "sunface");
        assert_eq!(t.2, "sunface");
        println!("Success!\nt: {:?}", t);
    }
    // Long tuples cannot be printed. Any more than 12 elements will not print
    pub fn exercise_003() {
        let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
        // println!("too long tuple: {:?}", too_long_tuple);
        println!("Unable to print or format too_long_tuple!");
    }
    // Destructuring tuple with pattern.
    pub fn exercise_004() {
        let tup = (1, 6.4, "hello");
        let (x, z, y) = tup;
        assert_eq!(x, 1);
        assert_eq!(y, "hello");
        assert_eq!(z, 6.4);
        println!("Success!\nx: {}\ny: '{}'\nz: {}", x, y, z);
    }
    // Destructure assignments.
    pub fn exercise_005() {
        let (x, y, z);
        (y, z, x) = (1, 2, 3);
        assert_eq!(x, 3);
        assert_eq!(y, 1);
        assert_eq!(z, 2);
        println!("Success!\nx: {}\ny: {}\nz: {}", x, y, z);
    }
    // Tuples can be used as function arguments and return values.
    pub fn exercise_006() {
        let (x, y) = sum_multiply((2, 3));
        assert_eq!(x, 5);
        assert_eq!(y, 6);
        println!("Succcess!\nx: {}\ny: {}",x , y);
        
        fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
            (nums.0 + nums.1, nums.0 * nums.1)
        }
    }
}

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

#[derive(Debug)]
pub struct Enums;
impl Enums {
    pub fn exercise_001() {
        #[derive(Debug)]
        enum Number {
            Zero,
            One, 
            Two,
        }
        #[derive(Debug)]
        enum Number1 {
            Zero = 0,
            One, 
            Two,
        }
        #[derive(Debug)]
        enum Number2 {
            Zero = 0,
            One = 1, 
            Two = 2,
        }
        // An enum variant can be converted to an integer by 'as' keyword.
        assert_eq!(Number::One as u8, Number1::One as u8);
        assert_eq!(Number1::One as u8, Number2::One as u8);
        println!("Number: {:?}", Number::One as u8)
    }
    pub fn exercise_002() {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32,)
        }
        // Each enum variant can hold its own data.
        let msg1 = Message::Move { x: 1, y: 2 };
        let msg2 = Message::Write(String::from("hello, world!"));
        println!("Success!\nmsg1: {:?}\nmsg2: {:?}", msg1, msg2);
    }
    pub fn exercise_003() {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32,)
        }
        // Can get the data from enum variant by pattern matching.
        let msg = Message::Move { x: 1, y: 1 };
        if let Message::Move { x: a, y: b } = msg {
            assert_eq!(a, b);
        } else {
            panic!("Never Let This Run!")
        }
        println!("Success!\nmsg: {:?}", msg);
    }
    pub fn exercise_004() {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32,)
        }
        fn show_message(msg: Message) {
            println!("msg: {:?}", msg);
        }
        let msgs: [Message; 3] = [
            Message::Quit,
            Message::Move { x: 1, y: 3 },
            Message::ChangeColor(255, 255, 0)
        ];
        for msg in msgs {
            show_message(msg)
        }
    }
    pub fn exercise_005() {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1)
            }
        }
        // Since there is no null in Rust, enum Option<T> is used to deal with cases when a 
        // value is absent.
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        if let Some(n) = six {
            println!("n: {:?}", n);
            println!("Success!");
        } else {
            panic!("NEVER LET THIS RUN!");
        }
    }
    pub fn exercise_006() {
        // Implement a Linked list using enums.
        use List::*;
        enum List {
            // Cons: Tuple struct that wraps an element and a pointer to the next node.
            Cons(u32, Box<List>),
            // Nil: A node that signifies the end of the linked list.
            Nil,
        }
        // Create an empty list.
        impl List {
            fn new() -> List {
                Nil
            }
            // Consume a list, and return the same list with a new element at is front
            fn prepend(self, elem: u32) -> List {
                // Cons also has type List
                Cons(elem, Box::new(self))
            }
            // Return the length of the list.
            fn len(&self) -> u32 {
                /* self has to be matche, because the behaviour of this method
                depends on the variant of self.
                Self has the type &List, and *self has the type List, matching on a 
                concrete type T is preferred over a match on a reference &T
                After Rust 2018 you can use self here and tail (with no ref) below as well
                rust will infer&s and ref tail. */
                match *self {
                    /* Can't take ownership of the tail, because self is borrowed;
                    Instead take a reference to the tail */ 
                    Cons(_, ref tail) => 1 + tail.len(),
                    // Base Case: An empty list has zero length
                    Nil => 0
                }
            }
            // Return representation of the list as a (heap allocated) string.
            fn stringify(&self) -> String {
                match *self {
                    Cons(head, ref tail) => {
                        /* format! is similar to print!, but returns a heap allocated string 
                        instead of printing to the console. */
                        format!("{}, {}", head, tail.stringify())
                    },
                    Nil => {
                        format!("Nil")
                    },
                }
            }
        }
        let mut list = List::new();
        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);
        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());
    }
}

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

pub struct PatternMatch;
impl PatternMatch {
    pub fn exercise_001() {
        #[derive(Debug)]
        enum Direction {
            East,
            West,
            North,
            South,
        }
        let dire = Direction::South;
        match dire {
            Direction::East => println!("East"),
            Direction::South | Direction::North => println!("South or North"),
            _ => println!("{:?}", dire),
        };
    }
    pub fn exercise_002() {
        let boolean = true;
        let binary = match boolean {
            true => 1,
            false => 0,
        };
        assert_eq!(binary, 1);
        println!("Success!\nboolean: {}", boolean);
    }
    pub fn exercise_003() {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32,)
        }
        let msgs = [
            Message::Quit,
            Message::Move { x: 1, y: 3 },
            Message::ChangeColor(255, 255, 0)
        ];
        for msg in &msgs {
            show_message(msg)
        }
        println!("Success!\nmsgs: {:?}", msgs);

        fn show_message(msg: &Message) {
            match *msg {
                Message::Move { x: a, y: b} => {
                    assert_eq!(a, 1);
                    assert_eq!(b, 3);
                },
                Message::ChangeColor(_, g, b) => {
                    assert_eq!(g, 255);
                    assert_eq!(b, 0);
                },
                _ => println!("no data in these variants")
            }
        }
    }
    pub fn exercise_004() {
        let aplphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];
        for ab in aplphabets {
            assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'))
        }
        println!("Success!\nalphabets: {:?}", aplphabets);
    }
    pub fn exercise_005() {
        #[derive(Debug)]
        enum MyEnum {
            Foo,
            Bar
        }
        let mut count = 0;
        let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
        for e in &v {
            if matches!(e, MyEnum::Foo) {
                count += 1;
            }
        }
        assert_eq!(count, 2);
        println!("Success!\nv: {:?}", v)
    }
    pub fn exercise_006() {
        let o = Some(7);
        match o {
            Some(i) => {
                println!("This is a really long string and `{:?}`", i);
                println!("Success!");
            },
            _ => {}
        };
        if let Some(i) = o {
            println!("This is a really long string and `{:?}`", i);
            println!("Success!");
        }
    }
    pub fn exercise_007() {
        enum Foo {
            Bar(u8)
        }
        let a = Foo::Bar(1);
        if let Foo::Bar(i) = a {
            println!("foobar holds the value: {}", i);
            println!("Success!");
        }
    }
    pub fn exercise_008() {
        enum Foo {
            Bar, 
            Baz, 
            Qux(u32)
        }
        let a = Foo::Qux(10);
        if let Foo::Bar = a {
            println!("mach foo::bar")
        } else if let Foo::Baz = a {
            println!("match foo::baz")
        } else {
            println!("match others")
        }
        // Or
        match a {
            Foo::Bar => println!("mach foo::bar"),
            Foo::Baz => println!("match foo::baz"),
            _ => println!("match others")
        }
    }
    // Shadowing.
    pub fn exercise_009() {
        let age = Some(30);
        if let Some(age) = age {
            assert_eq!(age, 30);
        }
        match age {
            Some(age) => println!("ages is a new variable, it's value is {}", age),
            _ => ()
        }
    }
}

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

pub struct Generics;
impl Generics {
    pub fn exercise_001() {
        struct A;
        struct S(A);
        struct SGen<T>(T);

        fn reg_fn(_s: S) {}
        fn gen_spec_t(_s: SGen<A>) {}
        fn gen_spec_i32(_s: SGen<i32>) {}
        fn generic<T>(_s: SGen<T>) {}

        reg_fn(S(A));
        gen_spec_t(SGen(A));
        gen_spec_i32(SGen(7));
        generic::<char>(SGen('A'));
        generic(SGen('Z'));

        println!("Success!");
    }
    pub fn exercise_002() {
        // T must implement the Add trait in order to use +.
        fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
            a + b
        }
        assert_eq!(5, sum(2i8, 3i8));
        assert_eq!(50, sum(20, 30));
        assert_eq!(2.46, sum(1.23, 1.23));
        println!("Success!")
    }
    pub fn exercise_003() {
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }

        let integer = Point {x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
        println!("Success!\ninteger: {:?}\nfloat: {:?}", integer, float);
    }
    pub fn exercise_004() {
        #[derive(Debug)]
        struct Point<T, U> {
            x: T,
            y: U
        }

        let p = Point { x: 5, y: "hello".to_string() };
        println!("Success!\np: {:?}", p)
    }
    pub fn exercise_005() {
        struct Val<T> {
            val: T,
        }
        impl<T> Val<T> {
            fn value(&self) -> &T {
                &self.val
            }
        }
        let x = Val { val: 3.0 };
        let y = Val { val: "hello". to_string() };
        println!("x: {}\ny: {}", x.value(), y.value());
    }
    pub fn exercise_006() {
        #[derive(Debug, Clone, Copy)]
        struct  Point<T, U> {
            x: T,
            y: U,
        }
        impl<T, U> Point<T, U> {
            fn mixup<V, W>(self, other: Point <V, W>) -> Point<T, W> {
                Point { x: self.x, y: other.y }
            }
        }
        let p1 = Point { x: 5, y: 10 };
        let p2 = Point { x: "Hello", y: '中' };
        let p3 = p1.mixup(p2);
        assert_eq!(p3.x, 5);
        assert_eq!(p3.y, '中');
        println!("Success!\np1: {:?}\np2: {:?}\np3: {:?}", p1, p2, p3);
    }
    pub fn exercise_007() {
        struct Point<T> {
            x: T,
            y: T,
        }
        impl Point<f64> {
            fn distance_from_origin(&self) -> f64 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
        let p = Point { x: 5.0, y: 10.0 };
        println!("p.distance_from_origing: {}", p.distance_from_origin())
    }
}

pub struct ConstGenerics;
impl ConstGenerics {
    pub fn exercise_001() {
        #[derive(Debug)]
        struct Array<T, const N: usize> {
            data: [T; N],
        }
        let arrays: [Array<i32, 3>; 3]  = [
            Array {
                data: [1, 2, 3],
            },
            Array {
                data: [1, 2, 3],
            },
            Array {
                data: [4, 5, 6]
            }
        ];
        let floats: [Array<f64, 2>; 3]  = [
            Array {
                data: [1.0, 2.0],
            },
            Array {
                data: [3.0, 4.0],
            },
            Array {
                data: [5.0, 6.0]
            }
        ];
        println!("Success!\narrays: {:?}\nfloats: {:?}", arrays, floats);
    }
}

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

pub struct CollectionTypes;
impl CollectionTypes {
    pub fn exercise_001() {
        let mut s: String = String::from("hello, ");
        s.push_str("world");
        s.push('!');
        assert_eq!(s, "hello, world!");
        move_ownership(s);
        println!("Success!");

        fn move_ownership(s: String) {
            println!("ownership of \"{}\" moved here!",s)
        }
    }
    pub fn exercise_002() {
        let mut s = String::from("hello, world");
        let slice1: &str = "hello, world";
        let slice1 = &s;
        let slice1 = s.as_str();
        let slice2 = &s[..=4];
        assert_eq!(slice2, "hello");
        let slice3: &mut String = &mut s.clone();
        let mut slice3: String = s.clone();
        slice3.push('!');
        assert_eq!(slice3, "hello, world!");
        println!("Success!\ns: '{}'\nslice1: '{}'\nslice2: '{}'\nslice3: '{}'", s, slice1, slice2, slice3);
    }
    pub fn exercise_003() {
        let s: String = String::from("hello, world!");
        let slice: &str = &s;
        let s: String = slice.to_string();
        assert_eq!(s, "hello, world!");
        println!("Success!\ns: '{}' and there were 2 alloactions to the heap.", s);
    }
    pub fn exercise_004() {
        let s = String::from("hello, 世界");
        let slice1 = &s[..=0];
        assert_eq!(slice1, "h");
        let slice2 = &s[7..10];
        assert_eq!(slice2, "世");
        for (i, c) in s.chars().enumerate() {
            if i == 7 {
                assert_eq!(c, '世')
            }
        }
        println!(
            "Success\ns: '{}'. 世界 is chinese shìjiè for world.\nslice1: '{}'\nslice2: '{}'", 
            s, slice1, slice2
        );
    }
    pub fn exercise_005() {
        // use utf8_slice to slice UTF8 string, can index chars instead of bytes.
        let mut s = String::new();
        s.push_str("hello");
        let v = vec![104, 101, 108, 108, 111];
        let s1 = String::from_utf8(v).unwrap();
        assert_eq!(s, s1);
        println!("Success!\ns: '{}'\ns1: '{}'", s, s1);
    }
    pub fn exercise_006() {
        // Avoids re-allocation of memory when string length is less than capacity.
        let mut s = String::with_capacity(25);
        println!("s.capacity(): {}", s.capacity());
        for _ in 0..2 {
            s.push_str("hello");
            println!("s.capacity(): {}", s.capacity());
        }
        println!("Success!\ns: '{}'", s);
    }
}

pub struct Vectors;
impl Vectors {
    pub fn exercise_001() {
        let arr: [u8; 3] = [1, 2, 3];
        let v = Vec::from(arr);
        is_vec(v);

        let v = vec![1, 2, 3];
        is_vec(v);

        let v = vec!(1, 2, 3);
        is_vec(v.clone());

        let v1 = Vec::from_iter(arr.iter().map(|x| *x as i32));
        is_vec(v1.clone());

        assert_eq!(v, v1);

        println!("Success!\narr: {:?}\nv: {:?}\nv1: {:?}", arr, v, v1);

        // Or

        let v: Vec<u8> = vec![1, 2, 3];
        is_vec(v);

        let v: Vec<u8> = vec!(1, 2, 3);
        is_vec(v.clone());

        let mut v1 = Vec::new();
        is_vec(v1.clone());

        for i in &v {
            v1.push(*i)
        }

        assert_eq!(v, v1);

        println!("Success!\narr: {:?}\nv: {:?}\nv1: {:?}", arr, v, v1);

        fn is_vec<T>(v: T) {
            
        }
    }
    pub fn exercise_002() {
        let mut v1 = Vec::from([1, 2, 4]);
        v1.pop();
        v1.push(3);
        let mut v2 = Vec::new();
        // v2.extend(v1.iter());
        // Or
        v2.extend(&v1);
        assert_eq!(v1, v2);
        println!("Success!\nv1: {:?}\nv2: {:?}", v1, v2);
    }
    pub fn exercise_003() {
        let arr = [1, 2, 3];
        let v1 = Vec::from(arr);
        let v2: Vec<i32> = arr.to_vec();
        // Or
        let v2: Vec<i32> = arr.into();
        assert_eq!(v1, v2);

        let s = "hello".to_string();
        let v1: Vec<u8> = s.into();

        let s = "hello".to_string();
        let v2 = s.into_bytes();
        assert_eq!(v1, v2);

        let s = "hello";
        let v3 = Vec::from(s);
        assert_eq!(v2, v3);

        let v4 : Vec<i32> = [0; 10].into_iter().collect();
        assert_eq!(v4, vec![0; 10]);
        println!(
            "Success!\narr: {:?}\nv1: {:?}\nv2: {:?}\nv3: {:?}\nv4: {:?}",
            arr, v1, v2, v3, v4
        );
    }
    pub fn exercise_004() {
        let mut v = Vec::from([1, 2, 3]);
        for i in 0..3 {
            println!("{:?}", v[i])
        }
        for i in 0..5 {
            if i < v.len() {
                v[i] += 1
            } else {
                v.push(i + 2)
            }
        }
        // Or
        let mut v = Vec::from([1, 2, 3]);
        for i in 0..5 {
            println!("{:?}", v.get(i))
        }
        for i in 0..5 {
            match v.get(i) {
                Some(e) => v[i] = e + 1,
                None => v.push(i + 2),
            }
        }
        assert_eq!(v, vec![2, 3, 4, 5, 6]);
        println!("Success!\nv: {:?}", v);
    }
    pub fn exercise_005() {
        let mut v = vec![1, 2, 3];
        let slice1 = &v[..];
        let slice2 = &v[..v.len()];
        assert_eq!(slice1, slice2);
        println!("v: {:?}\nslice1: {:?}\nslice2: {:?}", &v, slice1, slice2);

        let vec_ref: &mut Vec<i32> = &mut v;
        (*vec_ref).push(4);
        let slice3 = &v[0..];
        // slice3.push(4) can't because slices are immutable.
        assert_eq!(slice3, &[1, 2, 3, 4]);
        println!("Success!\nslice3: {:?}", slice3);
    }
    pub fn exercise_006() {
        // Vectors only reallocate when the len() greater than capacity.
        let mut vec = Vec::with_capacity(10);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 10);
        for i in 0..10 {
            vec.push(i)
        }
        assert_eq!(vec.len(), 10);
        assert_eq!(vec.capacity(), 10);
        println!("vec.len(): {}\nvec.capacity(): {}", vec.len(), vec.capacity());
        vec .push(11);
        assert_eq!(vec.len(), 11);
        assert!(vec.capacity() >= 11);
        println!("vec.len(): {}\nvec.capacity(): {}", vec.len(), vec.capacity());
        let mut vec = Vec::with_capacity(100);
        for i in 0..100 {
            vec.push(i)
        }
        assert_eq!(vec.len(), 100);
        assert_eq!(vec.capacity(), 100);
        println!("Success\nvec.len(): {}\nvec.capacity(): {}", vec.len(), vec.capacity());
    }
    pub fn exercise_007() {
        #[derive(Debug, PartialEq)]
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let v: Vec<IpAddr> = vec![IpAddr::V4("127.0.0.1".to_string()), IpAddr::V6("::1".to_string())];
        assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
        assert_eq!(v[1], IpAddr::V6("::1".to_string()));
        println!("Success!\nv: {:?}", v);
    }
    pub fn exercise_008() {
        trait IpAddr {
            fn display(&self);
        }
        struct V4(String);
        impl IpAddr for V4 {
            fn display(&self) {
                println!("ipv4: {:?}", self.0)
            }
        }
        struct V6(String);
        impl IpAddr for V6 {
            fn display(&self) {
                println!("ipv6: {:?}", self.0)
            }
        }
        let v: Vec<Box<dyn IpAddr>> = vec![
            Box::new(V4("127.0.0.1".to_string())),
            Box::new(V6("::1".to_string())),
        ];
        for ip in v {
            ip.display()
        }
    }
}

pub struct HashMaps;
impl HashMaps {
    pub fn exercise_001() {
        use std::collections::HashMap;
        let mut scores = HashMap::new();
        scores.insert("Sunface", 98);
        scores.insert("Daniel", 95);
        scores.insert("Ashley", 69);
        scores.insert("Katie", 58);
        let score = scores.get("Sunface");
        assert_eq!(score, Some(&98));
        if scores.contains_key("Daniel") {
            let score = scores["Daniel"];
            assert_eq!(score, 95);
            scores.remove("Daniel");
        }
        assert_eq!(scores.len(), 3);
        for (name, score) in scores {
            println!("The score of {} is {}", name, score);
        }
    }
    pub fn exercise_002() {
        use std::collections::HashMap;
        let teams = [
            ("Chinese Team", 100),
            ("American Team", 10),
            ("French Team", 50),
        ];
        let mut teams_map1 = HashMap::new();
        for team in &teams {
            teams_map1.insert(team.0, team.1);
        }
        let teams_map2 = HashMap::from(teams);
        assert_eq!(teams_map1, teams_map2);
        let teams_map3: HashMap<&str, i32> = teams.into_iter().collect();
        assert_eq!(teams_map1, teams_map3);
        println!(
            "Success!\nteams: {:?}\nteams_map1: {:?}\nteams_map2: {:?}\nteams_map3: {:?}", 
            teams, teams_map1, teams_map2, teams_map3
        )
    }
    pub fn exercise_003() {
        use std::collections::HashMap;
        let mut player_stats = HashMap::new();
        // Insert a key only if it doesn't already exist.
        player_stats.entry("health").or_insert(100);
        assert_eq!(player_stats["health"], 100);
        player_stats.entry("health").or_insert_with(random_stat_buff);
        assert_eq!(player_stats["health"], 100);
        let health = player_stats.entry("health").or_insert(50);
        assert_eq!(*health, 100);
        // Or
        assert_eq!(health, &100);
        *health -= 50;
        assert_eq!(*health, 50);
        println!("Success!\nhealth: {:?}", health);

        fn random_stat_buff() -> u8 { 42 }
    }
    pub fn exercise_004() {
        use std::collections::HashMap;
        #[derive(Debug, Eq, PartialEq, Hash)]
        struct Viking {
            name: String,
            country: String,
        }
        impl Viking {
            fn new(name: &str, country: &str) -> Viking {
                Viking { 
                    name: name.to_string(), 
                    country: country.to_string() 
                }
            }
        }
        let vikings = HashMap::from([
            (Viking::new("Einar", "Norway"), 25),
            (Viking::new("Olaf", "Denmark"), 24),
            (Viking::new("Herald", "Iceland"), 12),
        ]);
        for (viking, health) in &vikings {
            println!("{:?} has {} hp", viking, health);
        }
    }
    pub fn example_001() {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
        map.insert(1, 2);
        map.insert(3, 4);
        assert!(map.capacity() >= 100);
        map.shrink_to(50);
        map.shrink_to_fit();
        assert!(map.capacity() >= 2);
        println!("Success!\n{:?} has a capacity of {}", map, map.capacity());
    }
    pub fn exercise_005() {
        use std::collections::HashMap;
        let v1 = 10;
        let mut m1 = HashMap::new();
        m1.insert(v1, v1);
        println!("v1 is still usable after inserting to hashmap: {}", v1);
        let v2 = "hello".to_string();
        let mut m2 = HashMap::new();
        // v2 ownership moved but v1 copied.
        m2.insert(&v2, v1);
        assert_eq!(v2, "hello");
        println!("Success!\nv1: {:?}\nv2: {:?}", v1, v2);
    }
}

pub struct TypeCoercions;
impl TypeCoercions {
    pub fn exercise_001() {
        let decimal = 97.123_f32;
        let integer: u8 = decimal as u8;
        let c1: char = decimal as u8 as char;
        let c2 = integer as char;
        assert_eq!(integer + 1, 'b' as u8);
        println!("Success!\ndecimal: {}\ninteger: {}\nc1: {}\nc2: {}", decimal, integer, c1, c2);
    }
    #[allow(overflowing_literals)]
    pub fn exercise_002() {
        assert_eq!(u8::MAX, 255);
        let v: u8 = 1000 as u8;
        println!("Success!\nv: {}", v);
    }
    #[allow(overflowing_literals)]
    pub fn exercise_003() {
        assert_eq!(1000 as u16, 1000);
        assert_eq!(1000 as u8, 232);
        println!("1000 mod 256 is: {}", 1000 % 256);
        assert_eq!(-1_i8 as u8, 255);
        assert_eq!(300.1_f32 as u8, 255);
        assert_eq!(-100.1_f32 as u8, 0);
        unsafe {
            println!("300.0 as u8 is {}", 300.0_f32.to_int_unchecked::<u8>());
            println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
            println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
        }
    }
    pub fn exercise_004() {
        let mut values: [i32; 2] = [1, 2];
        let p1: *mut i32 = values.as_mut_ptr();
        let first_address: usize = p1 as usize;
        // 4 == std::mem::size_of::<i32>(), therefore increments the pointer address by 4 to get 2nd element.
        let second_address = first_address + 4;
        let p2: *mut i32 = second_address as *mut i32;
        unsafe {
           *p2 += 1;
        }
        assert_eq!(values[1], 3);
        println!(
            "Success!\nvalues{:?}\np1: {:?}\nfirst_address: {:?}\nsecond_address: {:?}\np2: {:?}",
            values, p1, first_address, second_address, p2
        );
    }
    pub fn exercise_005() {
        let arr: [u64; 13] = [0; 13];
        assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
        let a: *const [u64] = &arr;
        let b = a as *const [u8];
        unsafe {
            assert_eq!(std::mem::size_of_val(&*b), 1 * 13)
        }
        println!("Success!\narr: {:?}\na: {:?}\nb: {:?}", arr, a, b);
    }
}

pub struct FromIntoConversion;
impl FromIntoConversion {
    // When implementing From trait, into trait is automatically implemented.
    pub fn exercise_001() {
        let i1: i32 = false.into();
        let i2: i32 = i32::from(false);
        assert_eq!(i1, i2);
        assert_eq!(i1, 0);
        let i3: u32 = 'a'.into();
        let i3 = 'a' as i32;
        let s: String = 'a'.to_string();
        let s: String = String::from('a');
        let s: String = 'a'.into();
        println!("Success!\ni1: {}\ni2: {}\ni3: {}\ns: '{}'", i1, i2, i3, s);
    }
    pub fn exercise_002() {
        #[derive(Debug)]
        struct Number {
            value: i32,
        }
        impl From<i32> for Number {
            fn from(value: i32) -> Self {
                Self { value }
            }
        }
        let num = Number::from(30);
        assert_eq!(num.value, 30);
        let num: Number = 30.into();
        assert_eq!(num.value, 30);
        println!("Success!\nnum: {:?}", num);
    }
    pub fn exercise_003() {
        use std::{fs, io, num};
        #[derive(Debug)]
        enum CliError {
            IoError(io::Error),
            ParseError(num::ParseIntError),
        }
        impl From<io::Error> for CliError {
            fn from(value: io::Error) -> Self {
                Self::IoError(value)
            }
        }
        impl From<num::ParseIntError> for CliError {
            fn from(value: num::ParseIntError) -> Self {
                Self::ParseError(value)
            }
        }
        fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
            // ? auomaiclly converts io::Error to CliError.
            let contents = fs::read_to_string(&file_name)?;
            let num: i32 = contents.trim().parse()?;
            Ok(num)
        }
        println!("Success!\nFile not found: {:?}", open_and_parse_file("file_name"));
    }
    pub fn exercise_004() {
        let n: i16 = 256;
        let n: u8 = match n.try_into() {
            Ok(n) => n,
            Err(e) => {
                println!("there is an error when converting: {}, but we catch it", e.to_string());
                0
            }
        };
        assert_eq!(n, 0);
        println!("Success!\nn: {}", n);
    }
    pub fn exercise_005() {
        #[derive(Debug, PartialEq)]
        struct EvenNum(i32);
        impl TryFrom<i32> for EvenNum {
            type Error = ();
            fn try_from(value: i32) -> Result<Self, Self::Error> {
                if value % 2 == 0 {
                    Ok(EvenNum(value))
                } else {
                    Err(())
                }
            }
        }
        assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
        assert_eq!(EvenNum::try_from(5), Err(()));
        let result: Result<EvenNum, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNum(8)));
        let result: Result<EvenNum, ()> = 5i32.try_into();
        assert_eq!(result, Err(()));
        println!("Success\nresult: {:?}", result)
    }
}

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

pub struct Results;
impl Results {
    pub fn exercise_001() {
        use std::num::ParseIntError;
        fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
            let n1 = n1_str.parse::<i32>();
            let n2 = n2_str.parse::<i32>();
            Ok(n1.unwrap() * n2.unwrap())
        }
        let result = multiply("10", "2");
        assert_eq!(result, Ok(20));
        println!("result: {}", result.unwrap());
        let result = multiply(&('t' as u8).to_string(), "2");
        assert_eq!(result.as_ref().unwrap(), &232);
        println!("result: {}", result.unwrap());
        let result = multiply("4", "2");
        assert_eq!(result.as_ref().unwrap(), &8);
        println!("Success!\nresult: {}", result.unwrap());
    }
    pub fn exercise_002() {
        use std::num::ParseIntError;
        fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
            let n1 = n1_str.parse::<i32>()?;
            let n2 = n2_str.parse::<i32>()?;
            Ok(n1 * n2)
        }
        assert_eq!(multiply("3", "4").unwrap(), 12);
        println!("Success!\nmultiply(\"3\", \"4\").unwrap(): {}", multiply("3", "4").unwrap());
    }
    pub fn exercise_003() {
        use std::{fs::File, io::{self, Read, Error}};
        fn read_file1() -> Result<String, Error> {
            let f = File::open("hello.txt");
            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };
            let mut s = String::new();
            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }
        fn read_file2() -> Result<String, Error> {
            let mut s = String::new();
            File::open("hello.txt")?.read_to_string(&mut s)?;
            Ok(s)
        }
        assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
        println!(
            "Success!\nread_file1().unwrap_err().to_string(): {}", 
            read_file1().unwrap_err().to_string()
        )
    }
    pub fn exercise_004() {
        use std::num::ParseIntError;
        fn add_two_map(n_str: &str) -> Result<i32, ParseIntError> {
            n_str.parse::<i32>().map(|i| i + 2)
        }
        fn add_two_then(n_str: &str) -> Result<i32, ParseIntError> {
            n_str.parse::<i32>().and_then(|i| Ok(i + 2))
        }
        assert_eq!(add_two_map("4").unwrap(), 6);
        assert_eq!(add_two_then("4").unwrap(), 6);
        println!(
            "Success!\nadd_two_map(\"4\").unwrap(): {}\nadd_two_then(\"4\").unwrap(): {}", 
            add_two_map("4").unwrap(), add_two_then("4").unwrap()
        );
    }
    pub fn exercise_005() {
        use std::num::ParseIntError;
        fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
            match n1_str.parse::<i32>() {
                Ok(n1) => {
                    match n2_str.parse::<i32>() {
                        Ok(n2) => {
                            Ok(n1 * n2)
                        },
                        Err(e) => Err(e),
                    }
                },
                Err(e) => Err(e),
            }
        }
        fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
            n1_str
                .parse::<i32>()
                .and_then(
                    |n1| 
                    n2_str.parse::<i32>().map(|n2| n1 * n2)
                )
        }
        fn print(result: Result<i32, ParseIntError>) {
            match result {
                Ok(n) => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }
        let twenty = multiply1("10", "2");
        print(twenty);
        let tt = multiply("t", "2");
        print(tt);
        println!(
            "Success!\nmultiply(\"2\", \"3\").unwrap(): {}\nmultiply1(\"2\", \"3\").unwrap(): {}",
            multiply("2", "3").unwrap(), multiply1("2", "3").unwrap()
        );
    }
    pub fn exercise_006() {
        use std::num::ParseIntError;
        type Res<T> = Result<T, ParseIntError>;
        fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
            first_number_str.parse::<i32>().and_then(|first_number| {
                second_number_str
                    .parse::<i32>()
                    .map(|second_number| first_number * second_number)
            })
        }
        fn print(result: Res<i32>) {
            match result {
                Ok(n) => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }
        print(multiply("10", "2"));
        print(multiply("t", "2"));
        println!("Success!")
    }
    // main can also return a Result type which also allows the use of ? operator.
}

pub struct FormatPrint;
impl FormatPrint {
    pub fn exercise_001() {
        let s1 = "hello";
        let s = format!("{}, world!", s1);
        assert_eq!(s, "hello, world!");
        println!("s: '{}'", s);
    }
    pub fn exercise_002() {
        print!("hello world, ");
        println!("I am");
        println!("Sunface!");
    }
}

pub struct DebugDisplay;
impl DebugDisplay {
    pub fn exercise_001() {
        #[derive(Debug)]
        struct Structure(i32);
        println!("{} months in a year.", 12);
        println!("Now {:?} will print!", Structure(3));
    }
    pub fn exercise_002() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }
        let person = Person { name: "Sunface".to_string(), age: 18 };
        println!("{:#?}", person);
    }
    pub fn exercise_003() {
        use std::fmt;
        struct Structure(i32);
        struct Deep(Structure);
        impl fmt::Debug for Deep {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0.0)
            }
        }
        println!("Now {:?} will print!", Deep(Structure(7)));
    }
    pub fn exercise_004() {
        use std::fmt;
        struct Point2D {
            x: f64,
            y: f64,
        }
        impl fmt::Display for Point2D {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Display: {} + {}i", self.x, self.y)
            }
        }
        impl fmt::Debug for Point2D {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Debug: Complex {{ real: {}, imag: {} }}", self.x, self.y)
            }
        }
        let point = Point2D { x: 3.3, y: 7.2 };
        assert_eq!(format!("{}", point), "Display: 3.3 + 7.2i");
        assert_eq!(format!("{:?}", point), "Debug: Complex { real: 3.3, imag: 7.2 }");
        println!("Success!\npoint: {}\npoint: {:?}", point, point);
    }
    pub fn exercise_005() {
        use std::fmt;
        struct List(Vec<i32>);
        impl fmt::Display for List {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let vec = &self.0;
                write!(f, "[")?;
                for (count, v) in vec.iter().enumerate() {
                    if count != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", count, v )?;
                }
                write!(f, "]")
            }
        }
        let v = List(vec![1, 2, 3]);
        assert_eq!(format!("{}", v), "[0: 1, 1: 2, 2: 3]");
        println!("Success!\nv: {}", v);
    }
}

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

pub struct Closures;
impl Closures {
    pub fn exercise_001() {
        let color = String::from("green");
        // Ommitting move || .... before a closure will only hold an immutable references to color.
        // move || ... before a closure will move color.
        let print = || println!("`color`: {}", color);
        print();
        print();
        let _reborrow = &color;
        println!("{}", color);
    }
    pub fn exercise_002() {
        let mut count = 0;
        // adding move to closure will enable the Copy trait of an i32.
        let mut inc = move || {
            count += 1;
            println!("`count`: {}", count);
        };
        inc();
        let _reborrow = &count;
        inc();
        let _count_reborrowed = &mut count;
        assert_eq!(count, 0);
    }
    pub fn exercise_003() {
        let movable: Box<i32> = Box::new(3);
        let consume = || {
            println!("`movable`: {:?}", movable);
            take(&movable);
        };
        consume();
        consume();

        fn take<T>(_v: &T) {}
        // Or
        let consume = move || {
            println!("`movable`: {:?}", movable);
        };
        consume();
        consume();
    }
    pub fn exercise_004() {
        let example_closure = |x| x;
        let s = example_closure(String::from("hello"));
        let n = example_closure(5.to_string());
        println!("s: '{}'\nn: '{}'", s, n);
    }
    pub fn exercise_005() {
        fn fn_once<F>(func: F)
        where
            F: FnOnce(usize) -> bool,
        {
            println!("{}", func(3));
        }
        let x = vec![1, 2, 3];
        fn_once(|z| z == x.len());

        fn fn_once1<F>(func: F)
        where
            F: Fn(usize) -> bool,
        {
            println!("{}", func(3));
            println!("{}", func(4));
        }
        let x = vec![1, 2, 3];
        fn_once1(|z| z == x.len());
    }
    pub fn exercise_006() {
        let mut s = String::new();
        let update_string = |str| s.push_str(str);
        exec(update_string);
        println!("{:?}", s);

        fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
            f("hello")
        }
    }
    pub fn exercise_007() {
        fn apply<F>(f: F) where
            F: FnOnce() 
        {
            f();
        }
        fn apply_to_3<F>(f: F) -> i32 where
            F: Fn(i32) -> i32
        {
            f(3)
        }
        use std::mem;
        let greeting = "hello";
        let mut farewell = "goodbye".to_owned();
        let diary = || {
            println!("I said {}.", greeting);
            farewell.push_str("!!!");
            println!("Then I screamed {}.", farewell);
            println!("Now I can sleep. zzzzz");
            /* Manually calling drop forces `farewell` to 
            be captured by value. Now requires `FnOnce` */
            mem::drop(farewell);
        };
        apply(diary);
        let double = |x| 2 * x;
        println!("3 doubled: {}", apply_to_3(double));
    }
    pub fn exercise_008() {
        let mut s = String::new();
        let update_string = |str| -> String { 
            s.push_str(str); 
            println!("{}", s); 
            // FnMut can be used if not returning s.
            s 
        };
        exec(update_string);        

        fn exec<'a, F: FnOnce(&'a str) -> String>(mut f: F) {
            f("hello");
        }
    }
    pub fn exercise_009() {
        fn call_me<F: Fn()>(f: F) {
            f();
        }
        fn function() {
            println!("I'm a function!");
        }
        let closure = || println!("I'm a closure!");
        call_me(closure);
        call_me(function);
    }
    pub fn exercise_010() {
        fn static_dispatch() {
            // impl is a static dispatch.
            fn create_fn() -> impl Fn(i32) -> i32 {
                let num = 5;
                move |x| x + num
            }
            let fn_plain = create_fn();
            println!("Static Dispatch: {}", fn_plain(1));
        }
        static_dispatch();
        fn dynamic_dispatch() {
            // impl is a static dispatch.
            fn create_fn() -> Box<dyn Fn(i32) -> i32> {
                let num = 5;
                Box::new(move |x| x + num)
            }
            let fn_plain = create_fn();
            println!("Dynamic Dispatch: {}", fn_plain(1));
        }
        dynamic_dispatch();
    }
    pub fn exercise_011() {
        fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
            let num = 5;
            if x > 1 {
                Box::new(move |x| x + num)
            } else {
                Box::new(move |x| x + num)
            }
        }
        println!("Success!\nfactory(0)(1): {}", factory(0)(1));
    }
}

pub struct Iterators;
impl Iterators {
    pub fn exercise_001() {
        let arr = [0; 10];
        for i in arr {
            print!("{} ", i);
        }
    }
    pub fn exercise_002() {
        let mut v = Vec::new();
        for n in 0..100 {
            v.push(n)
        }
        assert_eq!(v.len(), 100);
        println!("v.len(): {}", v.len());
    }
    pub fn exercise_003() {
        let mut v1 = vec![1, 2].into_iter();
        assert_eq!(v1.next(), Some(1));
        assert_eq!(v1.next(), Some(2));
        assert_eq!(v1.next(), None);
        println!("Success!");
    }
    pub fn exercise_004() {
        let arr = vec![0; 10];
        for i in arr.iter() {
            println!("{}", i);
        }
        println!("{:?}", arr);
    }
    pub fn exercise_005() {
        let mut names = vec!["Bob", "Frank", "Ferris"];
        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }
        println!("names: {:?}", names);
    }
    pub fn exercise_006() {
        let mut values = vec![1, 2, 3];
        let mut values_iter = values.iter_mut();
        if let Some(v) = values_iter.next() {
            *v = 0
        }
        assert_eq!(values, vec![0, 2, 3]);
        println!("values: {:?}", values);
    }
    pub fn example_001() {
        #[derive(Debug)]
        struct Counter {
            count: u32,
        }
        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }
        impl Iterator for Counter {
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                if self.count < 5 {
                    self.count += 1;
                    Some(self.count)
                } else {
                    None
                }
            }
        }
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
        println!(
            "Counter::new(): {:?}\nCounter::new().next(): {:?}", 
            Counter::new(), Counter::new().next()
        );
    }
    pub fn exercise_007() {
        #[derive(Debug)]
        struct Fibonacci {
            curr: u32,
            next: u32,
        }
        impl Iterator for Fibonacci {
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                if self.curr + self.next < u32::MAX {
                    let forward = self.curr + self.next;
                    self.curr = self.next;
                    self.next = forward;
                    Some(self.curr)
                } else {
                    None
                }
            }
        }
        fn fibonnaci() -> Fibonacci {
            Fibonacci { curr: 0, next: 1 }
        }
        let mut fib = fibonnaci();
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
        println!("{:?}", fib.next());
    }
    pub fn exercise_008() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
        println!("v1: {:?}\ntotal: {}", v1, total);
    }
    pub fn exercise_009() {
        use std::collections::HashMap;
        let names = [("sunface", 18), ("sunfei", 18)];
        let folks: HashMap<&str, i32> = names.into_iter().collect();
        println!("{:?}", folks);
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<i32> = v1.into_iter().collect();
        assert_eq!(v2, vec![1, 2, 3]);
        println!("names: {:?}\nfolks: {:?}\nv2: {:?}", names, folks, v2);
    }
    pub fn exercise_010() {
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
        println!("v1: {:?}\nv2: {:?}", v1, v2);
    }
}
