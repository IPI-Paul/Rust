#![allow(unused)]
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
