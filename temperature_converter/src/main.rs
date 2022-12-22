use std::io;

fn main() {
  let mut temperature = String::new();
  let mut temperature_type = String::new();

  println!("Enter temperature");

  io::stdin()
    .read_line(&mut temperature)
    .expect("Failed to read line");
  
  let temperature: f64 = match temperature.trim().parse() {
    Ok(num) => num,
    Err(_) => 0.0,
  };

  println!("Enter type you want - 'c' = Celsius, 'f' = Fahrenheit");

  io::stdin()
    .read_line(&mut temperature_type)
    .expect("Failed to read line");

  let temperature_type: char = match temperature_type.trim().parse() {
    Ok(ch) => ch,
    Err(_) => 'c',
  };

  if temperature_type == 'c' {
    let result = (temperature - 32.0) * 5.0 / 9.0;
    println!("{}F = {}C", temperature, result);
  } else if temperature_type == 'f' {
    let result = (temperature * 1.8) + 32.0;
    println!("{}C = {}F", temperature, result);
  }
}
