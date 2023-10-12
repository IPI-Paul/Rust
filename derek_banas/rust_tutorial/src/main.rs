#![allow(unused)] 
// Prevents warnings about unused variables

use std::io::{self, BufRead};
use std::ops::Add;
use std::process::Output;
use rand::Rng;
use std::io::{Write, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::rc::Rc; 
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
mod restaurant;
use crate::restaurant::order_food;

fn main() {
    println!("Pick a Number to view the example
1)Hello World 2)Variable Types 3)Data Types 4)Booleans & Characters
5)Math Float Precision 6)Math operators 7)Random Values 8)Conditionals
9)Turnary operators 10)Match conditional 11)Arrays 12)Tuples 13) Strings
14)Casting 15)Enumerator types 16)Vectors 17)Functions 18)Generic types
19)Ownership 20)Hasmaps 21)Structs 22)Packages, Crates and Modules
23)panic macro 24)Read/Write & Error Handling 25)Iterators 26)Closures
27)Smart pointers 28)Concurrent 29)Concurrent with smart pointer
    ");
    let mut line = String::new();
    let input = io::stdin().read_line(&mut line);
    let choice: Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());
    match choice {
        Some(n) if n == 1 => example_1(),
        Some(n) if n == 2 => example_2(),
        Some(n) if n == 3 => example_3(),
        Some(n) if n == 4 => example_4(),
        Some(n) if n == 5 => example_5(),
        Some(n) if n == 6 => example_6(),
        Some(n) if n == 7 => example_7(),
        Some(n) if n == 8 => example_8(),
        Some(n) if n == 9 => example_9(),
        Some(n) if n == 10 => example_10(),
        Some(n) if n == 11 => example_11(),
        Some(n) if n == 12 => example_12(),
        Some(n) if n == 13 => example_13(),
        Some(n) if n == 14 => example_14(),
        Some(n) if n == 15 => example_15(),
        Some(n) if n == 16 => example_16(),
        Some(n) if n == 17 => example_17(),
        Some(n) if n == 18 => example_18(),
        Some(n) if n == 19 => example_19(),
        Some(n) if n == 20 => example_20(),
        Some(n) if n == 21 => example_21(),
        Some(n) if n == 22 => example_22(),
        Some(n) if n == 23 => example_23(),
        Some(n) if n == 24 => example_24(),
        Some(n) if n == 25 => example_25(),
        Some(n) if n == 26 => example_26(),
        Some(n) if n == 27 => example_27(),
        Some(n) if n == 28 => example_28(),
        Some(n) if n == 29 => example_29(),
        Some(_) => println!("Error"),
        None => {},
    }
}

// Concurrent with smart pointer.
fn example_29() {
    pub struct Bank {
        balance: f32
    }
    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!("Current Balance: {} Withdraw a smaller amount", bank_ref.balance);
        } else {
            bank_ref.balance -= amt;
            println!("Customer withdrew {} Current Balance {}", amt, bank_ref.balance);
        }
    }
    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00);
    }
    let bank: Arc<Mutex<Bank>> =Arc::new(Mutex::new(Bank {balance: 20.00}));
    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref)
        })
    });
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Total {}", bank.lock().unwrap().balance);
}

// Concurrent executes diferent blocks of code independently. Reuquires std::thread & std::time::Duration
// Parallel programming blocks code executes at the same time (threads).
// Common problems with parallel programming involve:
// 1. Threads are accessing data in the wrong order.
// 2. Threads are blocked from executing because of confussion over requirements to 
// proceed with execution.
fn example_28() {
    // No guarantees that these threads will execute.
    thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned threa: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..20 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    // To ensure threads execute...
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned threa: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..20 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    thread1.join().unwrap();
    pub struct Bank {
        balance: f32
    }
    fn withdraw(the_bank: &mut Bank, amt: f32) {
        the_bank.balance -= amt;
    }
    let mut bank = Bank{balance: 100.0};
    withdraw(&mut bank, 5.00);
    println!("Balance: {}", bank.balance);
    fn customer(the_bank: &mut Bank) {
        withdraw(the_bank, 5.00);
    }
    // Cannot have a thread otlive the main function. Need a smart pointer. Smart ponter needs
    // use std::rc::Rc use std::cell::RefCell use std::sync::{Arc, Mutex}
    // thread::spawn(|| {
    //     customer(&mut bank)
    // }).join().unwrap();
}

// Smart pointers are addresses to a location in memory. & sign operates as a pointer. Strings and 
// vectors are smart pointers. Ca be used to track the ownership of data.
fn example_27() {
    // Box stores data on the heap. Binary tree data structure utilising Box. 
    let b_int1 = Box::new(10);
    println!("b_int = {}", b_int1);
    // rust does not like null values. so, TreeNode<T> on it's own produces an error
    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            // Good practice to always have trailing commas.
            TreeNode { left: None, right: None, key, }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }
    let node1 = TreeNode::new(1)
        .left(TreeNode::new(2))
        .right(TreeNode::new(3));
}

// Closures are functions without names and more likely stored in a variable and can be used to pass 
// a function in to another function.
fn example_26() {
    // Basic type.
    // let var_name = |parameters| -> return_type {BODY}
    let can_vote = |age: i32| {
        age >= 18
    };
    println!("Can vote: {}", can_vote(8));
    let mut samp1 = 5;
    let print_var = || println!("samp1 = {}", samp1);
    print_var();
    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1 = {}", samp1);
    samp1 = 10;
    println!("samp1 = {}", samp1);
    fn use_func<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
        func(a, b)
    }
    let sum = |a, b| a + b;
    let prod = |a, b| a * b;
    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));
}

// Iterators are can help us cycle through values, vectors, arrays or maps.
// Cycles through by borrowing and not taking the values. Will not be able 
// to change values using .iter(), but using .into_iter() will consume the 
// collection and allow changes and the collection will no longer exist.
fn example_25() {
    let mut arr_it = [1,2,3,4];
    for val in arr_it.iter() {
        println!("{}", val);
    }
    // Create an iterator.
    let mut iter1 = arr_it.iter();
    println!("1st: {:?}", iter1.next());
}


// Reading and writing files combined with error handling.
fn example_24() {
    let path = "lines.txt";
    // Result has 2 variants Ok and Err
    // enum Result<T, E> { Ok(T), Err(E), }
    // Where T represents the data type of the value returned and E the type of error
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file: {:?}", error);
        }
    };
    write!(output, "Just some\nRandom words").expect("Failed to write to file");
    // Unwrap ignores the result and just gives the output from the file.
    let input = File::open(path).unwrap();
    // Buffer reads one line at a time.
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }
    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", e),
            },
            _other_error => panic!("Problem opening file: {:?}", error),
        },
    };
}

// panic!() for unrecoverable errors. Prints an error, memory is cleaned up then programe exits.
fn example_23() {
    panic!("Terrible Error");
    // The following will trigger a panic for index out of bounds.
    let lil_arr = [1, 2];
    // println!("{}", lil_arr[10]);
}

// Packages, Crates and Modules
// Split code into multiple files and packages can contain multiple crates which are just files with 
// code in it. You can define what code is public and what code is private.
// Creates new folder restaurant in src folder.
// Crates: Modules that produce a library or executable.
// Modules: Organise and handle privacy.
// Packages: Build, test and share crates. Can contain 0 or 1 library crates or can contain as many 
// binary crates as you may want. To create binary crates you will create a bin folder to save them in.
// Paths: A way of naming an item such as a structs and functions.
fn example_22() {
    order_food();
}

// A struct is a custom data type that is going to store multiple diferent types of data.
fn example_21() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }
    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("555 Main St"),
        balance: 234.50
    };
    bob.address = String::from("505 Main St");
    println!("{}", bob.address);
    struct Rectangle<T, U> {
        length: T,
        height: U,
    }
    let rec = Rectangle{length: 4, height: 10.5};
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
    struct Rectangle1 {length: f32, width: f32};
    struct Circle {length: f32, width: f32};
    const PI: f32 = 3.141592;
    impl Shape for Rectangle1 {
        fn new(length: f32, width: f32) -> Rectangle1 {
            return Rectangle1 {length, width};
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }
    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle {length, width};
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }
    let rec1: Rectangle1 = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);
    println!("Rec Area: {}", rec1.area());
    println!("Circ Area: {}", circ.area());
}

// Hashmaps are used to store key value pairs.
fn example_20() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flask", "Barry Allen");
    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v);
    }
    println!("Length: {}", heroes.len());
    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }
}

// Ownership
fn example_19() {
    // This will not apply to data types of integers, booleans, characters, floats, tuples.
    // It will apply with strings, arrays and vectors.
    let str1 = String::from("World");
    // Replaces ownership of str1. So, str1 no longer exists
    let str2 = str1;
    // Throws an error
    // println!("Hello {}", str1);
    let str1 = str2;
    let str2 = str1.clone();
    println!("Hello {}", str1);
    fn print_str(x:String) {
        println!("A string {}", x);
    }
    fn print_return_str(x: String) -> String {
        println!("A string {}", x);
        x
    }
    fn change_string(name: &mut String) {
        name.push_str(" is happy");
        println!("Message: {}", name);
    }
    print_str(str2);
    let str3 = print_return_str(str1);
    println!("str3 = {}", str3);
    let mut str4 = String::from("Derek");
    change_string(&mut str4);
}

// There are 2 types of ownership.
// Stack: Stores values in a last in first out format.
// Data on a stack must have a defined fixed size.
// Heap: When putting data on the heap you request a certain amount of space. The os finds
// space available and returns an address for tha space called a pointer.
// Rules: 
// 1. Each value has a variable that's called its owner.
// 2. There is only one owner at a time.
// 3.When the owner goes out of scope the value disappears.

// Can specify the data type to be used at a later time with generics
// and can be used with multiple diferent data types.
fn example_18() {
    // Try to add 2 values tha are not understood and return a value
    // of an unknown type.
    fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
        // Cannot use + on generics, but need use std::ops::Add;
        return x + y;
    }
    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("5.2 + 4.6 = {}", get_sum_gen(5.2, 4.6));
}

// Can define before or after fn main
fn example_17() {
    fn say_hello() {
        println!("Hello");
    }
    say_hello();
    // Pass arguments to a function
    fn get_sum(x: i32, y: i32) {
        println!("{} + {} = {}", x, y, x + y);
    }
    get_sum(5, 4);
    // Function that returns a value
    fn get_sum_2(x: i32, y: i32) -> i32 {
        x + y
    }
    println!("{}", get_sum_2(5, 4));
    get_sum(5, 4);
    // Function that returns a value
    fn get_sum_3(x: i32, y: i32) -> i32 {
        return x + y;
    }
    println!("{}", get_sum_3(5, 4));
    // Return more than one value
    fn get_2(x: i32) -> (i32, i32) {
        return (x + 1, x + 2);
    }
    let (val_1, val_2) = get_2(3);
    println!("Nums: {} {}", val_1, val_2);
    // Define a number list and pass the list to a function to be summed
    let num_list = vec![1,2,3,4,5];
    fn sum_list(list: &[i32]) -> i32 {
        let mut sum = 0;
        for &val in list.iter() {
            sum += &val;
        }
        sum
    }
    println!("Sum of list = {}", sum_list(&num_list))
}

// Vectors are like arrays and can grow if mutable but only contain
// values of the same type.
fn example_16() {
    // An empty vector
    let vec1: Vec<i32> = Vec::new();
    // Vector with defined values
    let mut vec2 = vec![1,2,3,4,];
    vec2.push(5);
    println!("1st: {}", vec2[0]);
    // Verify values exist
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd: {}", second),
        None => println!("No 2nd value"),
    }
    // Cycle trhough vector and apply operation
    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &vec2 {
        println!("{}", i);
    }
    println!("Vec length {}", vec2.len());
    // Returns the value that is removed
    println!("Pop: {:?}", vec2.pop());
}

// Enumerator types allow you to create custom data types that have a limited number of potential values
fn example_15() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }
    // Define functions for enumerator types
    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                // _ everything else
                _ => false
            }
        }
    }
    let today: Day = Day::Monday;
    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!("Donut day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay day"),
        Day::Friday => println!("Almost Weekend"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"),
    }
    println!("Is today the weekend {}", today.is_weekend());
}

// Casting
fn example_14() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    // Using AS keyword
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
    println!("u8: {} u8: {} u32: {}", int_u8, int2_u8, int3_u32);
}

// Strings have 2 types, String a vector that can be changed and &str strong reference string type
// that points to a string and allows viewing of the string.
fn example_13() {
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace() {
        println!("{}", word);
    }
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);
    // String of random different characters
    let st3 = String::from("x r t b h k k a m c");
    // Convert to a vector
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    // De-Duplicate
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }
    // String literal
    let st4: &str = "Random string";
    // Convert to a heap allocated string
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    // Convert in to an array of bytes
    let byte_arr1 = st5.as_bytes();
    // Get slice of a string
    let st6 = &st5[0..6];
    println!("String Length: {}", st6.len());
    // Delete values in a string if mutable
    st5.clear();
    // Combine strings
    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    // Not having & in front of the variable removes it from existance
    // and puts its contents in the new variable. The & gives us a 
    // reference to the variable it precedes.
    let st8 = st6 + &st7;
    // Output contents as unicode characters.
    for char in st8.bytes() {
        println!("{}", char)
    }
}

// Tuples can contain multiple different data types in a list of a fixed size
fn example_12() {
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
    println!("Name: {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Age: {}", v1)
}

// Arrays containing multiple different values and elements in an array
// must be of the same data type. Arrays have fixed size.
fn example_11() {
    let arr_1 = [1,2,3,4];
    println!("1st: {}", arr_1[0]);
    println!("Length: {}", arr_1.len());
    let arr_2 = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0{
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }
    let mut loop_idx = 0;
    while loop_idx < arr_2.len() {
        println!("Arr: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }
    let mut loop_idx = 0;
    for val in arr_2.iter() {
        println!("Val: {}", val);
    }
}

// Match conditional can be used for error handling
fn example_10() {
    let age2 = 8;
    match age2 {
        // Includes 18 in the range (1..18 only goes up to 17)
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an Important Birthday"),
    };
    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    };
}

// Turnary operators
fn example_9() {
    let mut my_age = 47;
    let can_vote = if my_age >= 18 {true} else {false};
    println!("Can Vote: {}", can_vote);
}

// Conditionals
fn example_8() {
    let age = 8;
    if (age >= 1) && (age <= 18) {
        println!("Important Birthday");
    } else if (age == 21) || (age == 50) {
        println!("Important Birthday");
    } else if age >= 65 {
        println!("Important Birthday");
    } else {
        println!("Not an Important Birthday");
    }
}

// Generate random values
fn example_7() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);
}

// Math operators
fn example_6() {
    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 as f32 / num_4 as f32);
    println!("5 % 4 = {}", num_3 % num_4);
    num_3 += 1;
}

// Math precision of floating point numbers
fn example_5() {
    // 32bit has 6 digits of precision
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111);
    // 64bit has 14 digits of precision
    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2 + 0.111111111111111);
}

// Boolean Data Types
fn example_4() {
    // to use a variable as a placeholder (not ready to be used)
    // precede it with an underscore and rust compiler will ignore it.
    let is_true = true;
    // With strings use double quotes and with Characters use single quotes.
    let my_grade = 'A';
}

// More specific on Data Types
fn example_3() {
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);
}

// Deep dive into variables
fn example_2() {
    // constants have Capital Case variable names. Use underscore to separate big values
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age += 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}

fn example_1() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    // returns an enum with 2 variable types Ok or Error
    io::stdin().read_line(&mut name)
        .expect("Didn't Receive Input");
    println!("Hello {}! {}", name.trim_end(), greeting);
}