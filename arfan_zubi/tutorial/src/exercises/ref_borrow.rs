#![allow(unused)]
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
