// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Besim";
  let mut age = 23;

  println!("My name is {} and I am {} years old", name, age);
  age = 24;
  println!("My name is {} and I am {} years old", name, age);

  // Define constant
  // while defining const you do have to define type
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // ASsign multiple vars
  let (my_name, my_age) = ("Besim", 24);

  println!("{} is {}", my_name, my_age);
}