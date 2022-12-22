use std::io;

fn main() {
  let mut num = String::new();

  println!("Please enter a fibonacci number");

  io::stdin()
    .read_line(&mut num)
    .expect("Failed to read line");


  let num: i32 = num.trim().parse().expect("Please enter a number");

  let result: u64 = fib(num);
  println!("{}", result);
}

fn fib(num: i32) -> u64 {
  if num == 0 {
    return 0;
  } else if num <= 2 {
    return 1;
  }

  let mut n1: u64 = 1;
  let mut n2: u64 = 1;
  
  let mut i = 3;

  while i <= num {
    let temp = n1;
    n1 = n2;
    n2 = temp + n1;
    i += 1;
  }

  return n2;
} 
