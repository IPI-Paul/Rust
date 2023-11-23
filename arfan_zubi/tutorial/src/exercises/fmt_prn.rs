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