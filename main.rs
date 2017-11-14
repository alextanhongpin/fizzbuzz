
fn main () {
  let mut fizzbuzz: i32 = 0;
  let mut fizz: i32 = 0;
  let mut buzz: i32 = 0;
  for i in 1..101 {
    match i {
      _ if i % 3 == 0 && i % 5 == 0 => {
        fizzbuzz += 1;
        println!("FizzBuzz");
      },
      _ if i % 3 == 0 => {
        fizz += 1;
        println!("Fizz");
      },
      _ if i % 5 == 0 => {
        buzz += 1;
        println!("Buzz");
      }
      _ => continue
    }
    println!("{}", i);
  }
  println!("found FizzBuzz = {} Fizz = {} Buzz = {}", fizzbuzz, fizz, buzz);
}