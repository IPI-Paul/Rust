#![allow(unused)]
#[derive(Debug)]
pub struct Enums;
impl Enums {
    pub fn exercise_001() {
        #[derive(Debug)]
        enum Number {
            Zero,
            One, 
            Two,
        }
        #[derive(Debug)]
        enum Number1 {
            Zero = 0,
            One, 
            Two,
        }
        #[derive(Debug)]
        enum Number2 {
            Zero = 0,
            One = 1, 
            Two = 2,
        }
        // An enum variant can be converted to an integer by 'as' keyword.
        assert_eq!(Number::One as u8, Number1::One as u8);
        assert_eq!(Number1::One as u8, Number2::One as u8);
        println!("Number: {:?}", Number::One as u8)
    }
    pub fn exercise_002() {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32,)
        }
        // Each enum variant can hold its own data.
        let msg1 = Message::Move { x: 1, y: 2 };
        let msg2 = Message::Write(String::from("hello, world!"));
        println!("Success!\nmsg1: {:?}\nmsg2: {:?}", msg1, msg2);
    }
    pub fn exercise_003() {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32,)
        }
        // Can get the data from enum variant by pattern matching.
        let msg = Message::Move { x: 1, y: 1 };
        if let Message::Move { x: a, y: b } = msg {
            assert_eq!(a, b);
        } else {
            panic!("Never Let This Run!")
        }
        println!("Success!\nmsg: {:?}", msg);
    }
    pub fn exercise_004() {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32,)
        }
        fn show_message(msg: Message) {
            println!("msg: {:?}", msg);
        }
        let msgs: [Message; 3] = [
            Message::Quit,
            Message::Move { x: 1, y: 3 },
            Message::ChangeColor(255, 255, 0)
        ];
        for msg in msgs {
            show_message(msg)
        }
    }
    pub fn exercise_005() {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1)
            }
        }
        // Since there is no null in Rust, enum Option<T> is used to deal with cases when a 
        // value is absent.
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        if let Some(n) = six {
            println!("n: {:?}", n);
            println!("Success!");
        } else {
            panic!("NEVER LET THIS RUN!");
        }
    }
    pub fn exercise_006() {
        // Implement a Linked list using enums.
        use List::*;
        enum List {
            // Cons: Tuple struct that wraps an element and a pointer to the next node.
            Cons(u32, Box<List>),
            // Nil: A node that signifies the end of the linked list.
            Nil,
        }
        // Create an empty list.
        impl List {
            fn new() -> List {
                Nil
            }
            // Consume a list, and return the same list with a new element at is front
            fn prepend(self, elem: u32) -> List {
                // Cons also has type List
                Cons(elem, Box::new(self))
            }
            // Return the length of the list.
            fn len(&self) -> u32 {
                /* self has to be matche, because the behaviour of this method
                depends on the variant of self.
                Self has the type &List, and *self has the type List, matching on a 
                concrete type T is preferred over a match on a reference &T
                After Rust 2018 you can use self here and tail (with no ref) below as well
                rust will infer&s and ref tail. */
                match *self {
                    /* Can't take ownership of the tail, because self is borrowed;
                    Instead take a reference to the tail */ 
                    Cons(_, ref tail) => 1 + tail.len(),
                    // Base Case: An empty list has zero length
                    Nil => 0
                }
            }
            // Return representation of the list as a (heap allocated) string.
            fn stringify(&self) -> String {
                match *self {
                    Cons(head, ref tail) => {
                        /* format! is similar to print!, but returns a heap allocated string 
                        instead of printing to the console. */
                        format!("{}, {}", head, tail.stringify())
                    },
                    Nil => {
                        format!("Nil")
                    },
                }
            }
        }
        let mut list = List::new();
        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);
        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify());
    }
}
