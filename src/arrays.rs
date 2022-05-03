// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
  let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

  // Re-assign value
  numbers[2] = 34;

  println!("{:?}", numbers);

  // Get single val
  println!("Single value {}", numbers[0]);

  // Get array length
  println!("Array Length: {}", numbers.len());

  // Arrays are stack allocated
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[1..4];

  println!("Slice: {:?}", slice);

  let other_slice: &[&str] = &["John", "Alice"];

  println!("Created slice: {:?}", other_slice);
}