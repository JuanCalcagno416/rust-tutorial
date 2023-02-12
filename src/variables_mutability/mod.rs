pub fn start_immutable() {
  // Declaring a variable like this will always be immutable, you cannot change the value
  let x = 5;
  println!("The value of x is: {x}");
  // x = 1; // this line will be commented so compiler doesn't complain
  println!("The value of x is: {x}");
}

pub fn start_mutable() {
  let mut x = 0;
  println!("The value of x is: {x}");
  x = 100;
  println!("The value of x is now: {x}");
}

pub fn lovely_constants() {
  const HELLO_WORLD:&str = "olis";
}

pub fn test_shadowing() {
  let x = 0;
  {
    let x = x + 10;
    println!("value of x inside of this block is: {x}");
  }
  println!("value of x is: {x}");
}
