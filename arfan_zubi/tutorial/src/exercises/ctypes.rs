#![allow(unused)]
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
