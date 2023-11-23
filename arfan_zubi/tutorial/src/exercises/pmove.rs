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
