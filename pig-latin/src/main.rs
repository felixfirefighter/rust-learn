use std::io;

fn main() {
    let mut words = String::new();

    println!("Enter strings");

    io::stdin()
        .read_line(&mut words)
        .expect("Failed to read line");

    let words = words.split(" ");

    for word in words {
      match word.chars().next().unwrap() {
        'a' | 'e' | 'i' | 'o' | 'u' => {
          println!("{}-hay", word)
        },
        other => {
          let first_part: String = word.chars().skip(1).collect();
          println!("{}-{}ay", first_part, other);
        }
      }
    }
}
