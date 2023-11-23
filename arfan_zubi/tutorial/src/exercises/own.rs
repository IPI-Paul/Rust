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
