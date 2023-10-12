/* 
Primitive str: Immutable fixed-length string somewhere in memory.
String: Growable, heap-allocated data structure. Use when you need to modify 
  or own string data.
*/

pub fn run() {
  let hello = "Hello";
  let mut or_hello = String::from("Hello ");
  // Get length
  println!("{}", or_hello.len());
  // For characters only (char).
  or_hello.push('W');
  // For strings (String).
  or_hello.push_str("orld!");
  println!("{:?}", (hello, &or_hello));
  // Capacity in bytes.
  println!("Capacity: {}", or_hello.capacity());
  // Check if empty.
  println!("Is Empty: {}", or_hello.is_empty());
  // Contains.
  println!("Contains Worls {}", or_hello.contains("World"));
  // Replace.
  println!("Raplace: {}", or_hello.replace("World", "There"));
  // Loop through string by whitespace.
  for word in or_hello.split_whitespace() {
    println!("{}", word);
  }
  // Create string with capacity.
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');
  println!("{}", s);
  // Assertion testing.
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());
}