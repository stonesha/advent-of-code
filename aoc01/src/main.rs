use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut sum_calories: Vec<i32> = Vec::new();

    let mut sum = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line == "" {
                    sum_calories.push(sum);
                    sum = 0;
                } else {
                    let number_read = i32::from_str(&line).unwrap();
                    sum += number_read;
                }
            }
            Err(err) => println!("{}", err),
        }
    }

    sum_calories.sort();

    if let Some(max) = sum_calories.last() {
        println!("Most calories carried is {} calories.", max);
    }

    let top_three_calories: i32 = sum_calories.iter().rev().take(3).sum();

    println!(
        "Tope three most calories carried is {} calories.",
        top_three_calories
    );

    Ok(())
}
