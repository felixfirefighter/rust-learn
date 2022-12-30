use std::collections::HashMap;
use std::io;

fn main() {
    let mut numbers = String::new();

    println!("Enter a list of integers, separated by whitespace");

    io::stdin()
        .read_line(&mut numbers)
        .expect("Failed to read line");

    let numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let numbers = sorted(numbers);

    if numbers.len() == 0 {
        println!("Median: {}", 0);
        println!("Mode: {}", 0);
        return;
    } else if numbers.len() == 1 {
        println!("Median: {}", numbers[0]);
        println!("Mode: {}", numbers[0]);
        return;
    }

    let mut median = 0;
    match numbers.len() % 2 {
        0 => {
            median = (numbers[numbers.len() / 2] + numbers[numbers.len() / 2 - 1]) / 2;
        }
        1 => median = numbers[numbers.len() / 2],
        _ => (),
    }

    let mut map = HashMap::new();

    for num in numbers {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut mode = 0;
    for (key, value) in map {
        if value > max {
            max = value;
            mode = key;
        }
    }

    println!("Median: {}", median);
    println!("Mode: {}", mode);
}

fn sorted<A, T>(array: A) -> A
where
    A: AsMut<[T]> + Clone,
    T: Ord,
{
    let mut array = array.clone();
    let slice = array.as_mut();
    slice.sort();

    array
}
