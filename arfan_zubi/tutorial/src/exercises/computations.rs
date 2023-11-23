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
