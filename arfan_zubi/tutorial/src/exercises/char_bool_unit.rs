#![allow(unused)]
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
