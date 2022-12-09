use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let marker = find_marker(&line);
                let start_of_message = find_start_of_message(&line);

                println!("Marker found: {}", marker);
                println!("Start of message found: {}", start_of_message);
            }
            Err(err) => println!("{}", err),
        }
    }

    Ok(())
}

fn find_start_of_message(signal: &String) -> usize {
    let mut start_of_message = String::new();

    for (i, c) in signal.chars().enumerate() {
        if start_of_message.len() == 14 {
            return i;
        } else if start_of_message.contains(c) {
            let duplicate_index = start_of_message.find(c).unwrap() + 1;
            let length = start_of_message.len();

            start_of_message = start_of_message[duplicate_index..length].to_string();
            start_of_message.push(c);
        } else {
            start_of_message.push(c);
        }
    }

    0
}

fn find_marker(signal: &String) -> usize {
    let mut marker = String::new();

    for (i, c) in signal.chars().enumerate() {
        if marker.len() == 4 {
            return i;
        } else if marker.contains(c) {
            marker.clear();
        } else {
            marker.push(c);
        }
    }

    0
}
