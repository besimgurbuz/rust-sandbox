// Conditionals - Used to check the condition of something and act on the result

fn bartender(age: u8) -> impl FnMut() {
  let mut check_id: bool = false;

  move || {
    if age >= 21 && check_id {
      println!("Bartender: What would you like to dring?");
    } else if age < 21 && check_id {
      println!("Bartender: Sorry, you have to leave.");
    } else {
      println!("Bartender: I'll need to see your ID");
      check_id = true;
    }
  }
}

pub fn run() {
  let age: u8 = 18;

  // If/Else
  let mut ask_for_drink = bartender(age);

  ask_for_drink();
  ask_for_drink();

  // Shorthand if
  let is_of_age = if age >= 21 { true } else { false };

  println!("Is of age: {}", is_of_age);
}