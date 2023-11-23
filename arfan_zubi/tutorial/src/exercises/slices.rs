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
