// Vectors - Resizable arrays

use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

  // Re-assign value
  numbers[2] = 34;

  // Add on to vector
  numbers.push(6);
  numbers.push(8);

  // Pop off last value
  numbers.pop();

  println!("{:?}", numbers);

  // Get single val
  println!("Single value {}", numbers[0]);

  // Get vector length
  println!("Vector Length: {}", numbers.len());

  // Arrays are stack allocated
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[1..4];

  println!("Slice: {:?}", slice);

  // Loop through vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop & mutate values
  for x in numbers.iter_mut() {
    *x *= 2;
  }

  println!("Numbers Vec: {:?}", numbers);
}