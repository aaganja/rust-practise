fn main() {
  let mut x = 45; 
  
  if x == 30 {
    println!("it value is 30");
  }
  else if x > 45 {
    println!("its value is greater that 45");
  }
  else {
    println!("it value is not 30");
  }

  println!("The value of x is {}", x);
  
  x = 60;

  println!("The value of x is {}", x);
}
