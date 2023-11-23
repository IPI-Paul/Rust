#![allow(unused, irrefutable_let_patterns)]
pub struct PatternMatch;
impl PatternMatch {
    pub fn exercise_001() {
        #[derive(Debug)]
        enum Direction {
            East,
            West,
            North,
            South,
        }
        let dire = Direction::South;
        match dire {
            Direction::East => println!("East"),
            Direction::South | Direction::North => println!("South or North"),
            _ => println!("{:?}", dire),
        };
    }
    pub fn exercise_002() {
        let boolean = true;
        let binary = match boolean {
            true => 1,
            false => 0,
        };
        assert_eq!(binary, 1);
        println!("Success!\nboolean: {}", boolean);
    }
    pub fn exercise_003() {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32,)
        }
        let msgs = [
            Message::Quit,
            Message::Move { x: 1, y: 3 },
            Message::ChangeColor(255, 255, 0)
        ];
        for msg in &msgs {
            show_message(msg)
        }
        println!("Success!\nmsgs: {:?}", msgs);

        fn show_message(msg: &Message) {
            match *msg {
                Message::Move { x: a, y: b} => {
                    assert_eq!(a, 1);
                    assert_eq!(b, 3);
                },
                Message::ChangeColor(_, g, b) => {
                    assert_eq!(g, 255);
                    assert_eq!(b, 0);
                },
                _ => println!("no data in these variants")
            }
        }
    }
    pub fn exercise_004() {
        let aplphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];
        for ab in aplphabets {
            assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'))
        }
        println!("Success!\nalphabets: {:?}", aplphabets);
    }
    pub fn exercise_005() {
        #[derive(Debug)]
        enum MyEnum {
            Foo,
            Bar
        }
        let mut count = 0;
        let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
        for e in &v {
            if matches!(e, MyEnum::Foo) {
                count += 1;
            }
        }
        assert_eq!(count, 2);
        println!("Success!\nv: {:?}", v)
    }
    pub fn exercise_006() {
        let o = Some(7);
        match o {
            Some(i) => {
                println!("This is a really long string and `{:?}`", i);
                println!("Success!");
            },
            _ => {}
        };
        if let Some(i) = o {
            println!("This is a really long string and `{:?}`", i);
            println!("Success!");
        }
    }
    pub fn exercise_007() {
        enum Foo {
            Bar(u8)
        }
        let a = Foo::Bar(1);
        if let Foo::Bar(i) = a {
            println!("foobar holds the value: {}", i);
            println!("Success!");
        }
    }
    pub fn exercise_008() {
        enum Foo {
            Bar, 
            Baz, 
            Qux(u32)
        }
        let a = Foo::Qux(10);
        if let Foo::Bar = a {
            println!("mach foo::bar")
        } else if let Foo::Baz = a {
            println!("match foo::baz")
        } else {
            println!("match others")
        }
        // Or
        match a {
            Foo::Bar => println!("mach foo::bar"),
            Foo::Baz => println!("match foo::baz"),
            _ => println!("match others")
        }
    }
    // Shadowing.
    pub fn exercise_009() {
        let age = Some(30);
        if let Some(age) = age {
            assert_eq!(age, 30);
        }
        match age {
            Some(age) => println!("ages is a new variable, it's value is {}", age),
            _ => ()
        }
    }
}
