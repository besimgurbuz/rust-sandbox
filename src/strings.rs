// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
  let hello = "Hello"; // immutable fixed length
  let mut growable_hello = String::from("Hello");

  // get length
  println!("Length {}", hello.len());
  println!("Growable Length {}", growable_hello.len());

  // for growable we can push more! (for char!)
  growable_hello.push(' ');
  growable_hello.push('W');

  // pushing strings
  growable_hello.push_str("orld!");

  // Capacity in bypes
  println!("Capacity: {}", growable_hello.capacity());

  // Check if empty
  println!("Is empty: {}", growable_hello.is_empty());

  // Contains
  println!("Contains 'World' {}", growable_hello.contains("World"));

  // Replace
  println!("Replace: {}", growable_hello.replace("World", "There"));

  // Loop throuh string by whitespace
  for word in growable_hello.split_whitespace() {
    println!("{}", word);
  }

  // Create string whith capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  // Assertion testing
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());

  println!("{}", s);
}