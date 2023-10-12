use std::io::stdin;

pub(crate) fn main() {
  println!("Pick a Number to view the example 
1)Hello World 2)Variable Types 3)Storing & Placing variables 4)Change outputs
5)Math functions 6)Absolute values 7)Conditionals 8)Looping 9)Strings 10)Receiving and manipulating input
11)Arrays 12)Vectors 13)Tuples 14)Functions 15)Closures 16)References 17)Structs & Traits
18)Enumerator Types
  ");
  let mut line = String::new();
  let input = stdin().read_line(&mut line);
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
      Some(_) => println!("Error"),
      None => {},
  }
}

// Enumerator types
fn example_18() {
  enum Hero {
    Fast,
    Strong(i32),
    Info {name: String, secret: String}
  }
  fn get_info(h: Hero) {
    match h {
        Hero::Fast => println!("Fast"),
        Hero::Strong(i) => println!("Lifts {} tons", i),
        Hero::Info { name, secret } => println!("{} is {}", name, secret),
    }
  }
  let hulk = Hero::Strong(100);
  let quicksilver = Hero::Fast;
  let spiderman = Hero::Info { name: "Spiderman".to_owned(), secret: "Peter Parker".to_owned() };
  get_info(hulk);
  get_info(spiderman);
}

// Structs can be used to create custom data types
fn example_17() {
  struct Circle {
    x: f64,
    y: f64,
    radius: f64,
  }
  // Better way to create a function tied to a struct
  fn get_radius(circle: &Circle) -> f64 {
    circle.radius
  }
  // Recommended way to create a function tied to a struct is to implement it
  impl Circle {
    pub fn get_x(&self) -> f64 {
      self.x
    }
  }  
  let mut circle1 = Circle {
    x: 10.0, y: 10.0, radius: 10.0
  };
  println!("X: {}, Y: {}, R: {}", circle1.x, circle1.y, circle1.radius);
  println!("Circle Radius: {}", get_radius(&circle1));
  println!("Circle X: {}", circle1.get_x());
  // Traits
  struct Rectangle {
    height: f64,
    width: f64,
  }
  trait HasArea {
    fn area(&self) -> f64;
  }
  impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.14159 * (self.radius * self.radius)
    }
  }
  impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }
  }
  println!("Circle Area: {}", circle1.area());
  let mut rect1 = Rectangle {
    height: 10.0, width: 10.0
  };
  println!("Rect Area: {}", rect1.area())
}

// References. Will only have one binding for each resource
fn example_16() {
  // Causes error because below are non-primitive
  // let vect1 = vec![1, 2, 3];
  // let vect2 = vect1;
  // println!("vect1[0]: {}", vect1[0])
  // Uses primitive types
  let prim_val = 1;
  let prim_val2 = prim_val;
  println!("prim_val: {}", prim_val);
  fn sum_vects(v1: &Vec<i32>) -> i32 {
    let sum = v1.iter().fold(0, |mut sum, &x| {sum += x; sum});
    return sum;
  }
  let vect2 = vec![1, 2, 3];
  println!("Sum of Vect: {}", sum_vects(&vect2));
  println!("Vect: {:?}", vect2);
}

// Closures represent blocks of code
fn example_15() {
  let sum_nums = |x: i32, y: i32| x + y;
  println!("7 + 8 = {}", sum_nums(7, 8));
  let num_ten = 10;
  let add_10 = |x: i32| x + num_ten;
  println!("5 + 10 = {}", add_10(5));
}

//  Functions
fn example_14() {
  fn say_hello(name: &str) {
    println!("Hello {}", name);
  }
  say_hello("Derek");
  fn get_sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
  }
  println!("5 + 4 = {}", get_sum(5, 4));
  let sum = get_sum;
  println!("6 + 4 = {}", sum(6, 4));
}

// Tuples are fixed sized lists
fn example_13() {
  let rand_tuple = ("Derek", 40);
  let rand_tuple_2: (&str, i8) = ("Derek", 40);
  println!("Name: {}", rand_tuple_2.0);
}

// Vectors are lists that can grow and shrink
fn example_12() {
  let mut vect1 = vec![1, 2, 3, 4, 5];
  println!("Item 2: {}", vect1[1]);
  // Iterate Vectors or Arrays
  for i in &vect1 {
    println!("{}", i);
  }
  vect1.push(6);
  vect1.pop();
}

// Arrays
fn example_11() {
  let rand_array = [1, 2, 3];
  println!("{}", rand_array[0]);
  println!("{}", rand_array.len());
  println!("Second 2: {:?}", &rand_array[1..3]);
}

// Receive input from user and manipulate it
fn example_10() {
  'outer: loop {
      let number = 10;
      println!("Pick a Number");
      loop {
          let mut line = String::new();
          let input = stdin().read_line(&mut line);
          let guess: Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());
          match guess {
              None => println!("Enter a Number"),
              Some(n) if n == number => {
                println!("You Guessed it");
                break 'outer;
              }
              Some(n) if n < number => println!("Too Low"),
              Some(n) if n > number => println!("Too High"),
              Some(_) => println!("Error")
          }
      }
  }
}

// Strings
fn example_9() {
  let rand_string = "I am a random string";
  println!("Length: {}", rand_string.len());
  let (first, second) = rand_string.split_at(6);
  println!("First: {} Second: {}", first, second);
  let count = rand_string.chars().count();
  let mut chars = rand_string.chars();
  let mut indiv_char = chars.next();
  loop {
    match indiv_char {
      Some(x) => println!("{}",x),
      None => break,
    }
    indiv_char = chars.next();
  }
  let mut iter = rand_string.split_whitespace();
  let mut indiv_word = iter.next();
  loop {
    match indiv_word {
      Some(x) => println!("{}", x),
      None => break,
    }
    indiv_word = iter.next();
  }
  let rand_string2 = "I am a random 
  string\nThere are other strings like it\nThis
  string is the best";
  let mut lines = rand_string2.lines();
  let mut indiv_line = lines.next();
  loop {
    match indiv_line {
        Some(x) => println!("{}", x),
        None => break,
    }
    indiv_line = lines.next();
  }
  println!("Find Best: {}", rand_string2.contains("best"))
}

// Looping
fn example_8() {
  let mut x = 1;
  loop {
    if x % 2 == 0 {
      println!("{}", x);
      x += 1;
      continue;
    }
    if x > 10 {
      break;
    }
    x += 1;
    continue;
  }
  let mut y = 1;
  while y <= 10 {
      println!("{}", y);
      y += 1;
  }
  // Work with a range
  for z in 1..10 {
    println!("For: {}",z);
  }
}

// Conditionals
fn example_7() {
  // != == > < >= <=
  // && || !
  let age_old = 6;
  if(age_old == 5) {
    println!("Go to Kindergarten");
  } else if(age_old > 5) && (age_old <= 18) {
    println!("Go to grade {}", (age_old - 5));
  } else if (age_old <= 25) && (age_old > 18) {
      println!("Go to College");
  } else {
      println!("Do what you want");
  }
  println!("!true = {}", !true);
  println!("true || false = {}", true || false);
  println!("true != false: {}", (true != false));
  // no default turnary operator, but you can make one
  let can_vote = if (age_old >= 18) {true} else {false};
  println!("Can Vote: {}", can_vote);
}

fn example_6() {
  // Define data type as a mutable variable
  let mut neg_4 = -4i32;
  // Get the absolute value
  println!("abs(-4) = {}", neg_4.abs());
  println!("4 ^ 6 = {}", 4i32.pow(6));
  println!("sqrt 9 = {}", 9f64.sqrt());
  println!("cbrt 9 = {}", 27f64.cbrt());
  println!("Round 1.45 = {}", 1.45f64.round());
  println!("Floor 1.45 = {}",1.45f64.floor());
  println!("Ceiling 1.45 = {}", 1.45f64.ceil());
  println!("e ^ 2 = {}", 2f64.exp());
  println!("log(2) = {}", 2f64.ln());
  println!("log10(2) = {}", 2f64.log10());
  println!("90 to radians = {}", 90f64.to_radians());
  println!("PI to Degrees = {}", 3.14f64.to_degrees());
  println!("Max 4, 5 = {}", 4f64.max(5f64));
  println!("Min 4, 5 = {}", 4f64.min(5f64));
  println!("Sin 3.14 = {}", 3.14f64.sin());
}

// Math functions available
fn example_5() {
  println!("5 + 4 = {}", 5 + 4);
  println!("5 - 4 = {}", 5 - 4);
  println!("5 * 4 = {}", 5 * 4);
  println!("5 / 4 = {}", 5f32 / 4f32);
  println!("5 % 4 = {}", 5 % 4);
}

fn example_4() {
  // Change the output in different ways
  println!("{:.2}", 1.234);
  // Able to output binary hexidecimal and float information
  println!("B: {:b} H: {:x} O: {:o}", 10, 10, 10);
  // Use named arguments and define whitespace
  println!("{ten:>ws$}", ten=10, ws=5);
  // pad arguments with 0/zero
  println!("{ten:>0ws$}", ten=10, ws=5);
}

fn example_3() {
  // Tell rust the variable type. mut meaning mutable.
  let mut age: i32 = 40;
  // Storing boolean data types
  let is_it_true: bool = true;
  // Storing individual characters
  let let_x: char = 'x';
  // Place vairaibles inside of output
  println!("I am {} years old", age);
  // set the values for multiple variables. Variable names should be 
  // separated with an underscore
  let (f_name, l_name) = ("Derek", "Banas");
  // Place mutliple different pieces of information in diferrent ways
  println!("It is {0} that {1} is {0}", is_it_true, let_x);
}

fn example_2() {
  // (i)nteger (u)nsigned (f)loat buffer(size)
  use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

  // Let rust decide the variable type.
  let _num = 10; 

  println!("Max i8 {}", i8::MAX);
  println!("Min i8 {}", i8::MIN);
  println!("Max i16 {}", i16::MAX);
  println!("Min i16 {}", i16::MIN);
  println!("Max i32 {}", i32::MAX);
  println!("Min i32 {}", i32::MIN);
  println!("Max i64 {}", i64::MAX);
  println!("Min i64 {}", i64::MIN);
  println!("Max isize {}", isize::MAX);
  println!("Min isize {}", isize::MIN);
  println!("Max usize {}", usize::MAX);
  println!("Min usize {}", usize::MIN);
  println!("Max f32 {}", f32::MAX);
  println!("Min f32 {}", f32::MIN);
  println!("Max f64 {}", f64::MAX);
  println!("Min f64 {}", f64::MIN);
}

fn example_1() {
  println!("Hello World");
}