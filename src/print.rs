pub fn run() {
  println!("Hello from print.rs file");

  // Basic formatting
  println!("{} is from {}", "Besim", "Turkey");

  // Positional Arguments
  println!("{0} is from {1} and {0} likes {2}", "Besim", "Turkey", "code");

  // Named Arguments
  println!("{name} likes play {activity}", name = "Besim", activity = "Basketball");

  // Placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // Placeholder for debug trait
  println!("{:?}", (12, true, "hello"));

  // Basic math
  println!("10 + 10 = {}", 10 + 10);
}