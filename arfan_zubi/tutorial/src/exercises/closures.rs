#![allow(unused)]
pub struct Closures;
impl Closures {
    pub fn exercise_001() {
        let color = String::from("green");
        // Ommitting move || .... before a closure will only hold an immutable references to color.
        // move || ... before a closure will move color.
        let print = || println!("`color`: {}", color);
        print();
        print();
        let _reborrow = &color;
        println!("{}", color);
    }
    pub fn exercise_002() {
        let mut count = 0;
        // adding move to closure will enable the Copy trait of an i32.
        let mut inc = move || {
            count += 1;
            println!("`count`: {}", count);
        };
        inc();
        let _reborrow = &count;
        inc();
        let _count_reborrowed = &mut count;
        assert_eq!(count, 0);
    }
    pub fn exercise_003() {
        let movable: Box<i32> = Box::new(3);
        let consume = || {
            println!("`movable`: {:?}", movable);
            take(&movable);
        };
        consume();
        consume();

        fn take<T>(_v: &T) {}
        // Or
        let consume = move || {
            println!("`movable`: {:?}", movable);
        };
        consume();
        consume();
    }
    pub fn exercise_004() {
        let example_closure = |x| x;
        let s = example_closure(String::from("hello"));
        let n = example_closure(5.to_string());
        println!("s: '{}'\nn: '{}'", s, n);
    }
    pub fn exercise_005() {
        fn fn_once<F>(func: F)
        where
            F: FnOnce(usize) -> bool,
        {
            println!("{}", func(3));
        }
        let x = vec![1, 2, 3];
        fn_once(|z| z == x.len());

        fn fn_once1<F>(func: F)
        where
            F: Fn(usize) -> bool,
        {
            println!("{}", func(3));
            println!("{}", func(4));
        }
        let x = vec![1, 2, 3];
        fn_once1(|z| z == x.len());
    }
    pub fn exercise_006() {
        let mut s = String::new();
        let update_string = |str| s.push_str(str);
        exec(update_string);
        println!("{:?}", s);

        fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
            f("hello")
        }
    }
    pub fn exercise_007() {
        fn apply<F>(f: F) where
            F: FnOnce() 
        {
            f();
        }
        fn apply_to_3<F>(f: F) -> i32 where
            F: Fn(i32) -> i32
        {
            f(3)
        }
        use std::mem;
        let greeting = "hello";
        let mut farewell = "goodbye".to_owned();
        let diary = || {
            println!("I said {}.", greeting);
            farewell.push_str("!!!");
            println!("Then I screamed {}.", farewell);
            println!("Now I can sleep. zzzzz");
            /* Manually calling drop forces `farewell` to 
            be captured by value. Now requires `FnOnce` */
            mem::drop(farewell);
        };
        apply(diary);
        let double = |x| 2 * x;
        println!("3 doubled: {}", apply_to_3(double));
    }
    pub fn exercise_008() {
        let mut s = String::new();
        let update_string = |str| -> String { 
            s.push_str(str); 
            println!("{}", s); 
            // FnMut can be used if not returning s.
            s 
        };
        exec(update_string);        

        fn exec<'a, F: FnOnce(&'a str) -> String>(mut f: F) {
            f("hello");
        }
    }
    pub fn exercise_009() {
        fn call_me<F: Fn()>(f: F) {
            f();
        }
        fn function() {
            println!("I'm a function!");
        }
        let closure = || println!("I'm a closure!");
        call_me(closure);
        call_me(function);
    }
    pub fn exercise_010() {
        fn static_dispatch() {
            // impl is a static dispatch.
            fn create_fn() -> impl Fn(i32) -> i32 {
                let num = 5;
                move |x| x + num
            }
            let fn_plain = create_fn();
            println!("Static Dispatch: {}", fn_plain(1));
        }
        static_dispatch();
        fn dynamic_dispatch() {
            // impl is a static dispatch.
            fn create_fn() -> Box<dyn Fn(i32) -> i32> {
                let num = 5;
                Box::new(move |x| x + num)
            }
            let fn_plain = create_fn();
            println!("Dynamic Dispatch: {}", fn_plain(1));
        }
        dynamic_dispatch();
    }
    pub fn exercise_011() {
        fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
            let num = 5;
            if x > 1 {
                Box::new(move |x| x + num)
            } else {
                Box::new(move |x| x + num)
            }
        }
        println!("Success!\nfactory(0)(1): {}", factory(0)(1));
    }
}
