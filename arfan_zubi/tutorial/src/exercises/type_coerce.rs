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
