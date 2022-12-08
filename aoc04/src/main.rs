use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut subranges_found = 0;
    let mut overlaps_found = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let split = line.split_once(',');

                if let Some(split) = split {
                    let first_range = convert_tuple(split.0.split_once('-').unwrap());
                    let second_range = convert_tuple(split.1.split_once('-').unwrap());

                    if check_subrange(first_range, second_range) {
                        subranges_found += 1;
                    }

                    if check_overlap(first_range, second_range) {
                        println!("{:?} {:?}", first_range, second_range);
                        overlaps_found += 1;
                    }
                }
            }
            Err(err) => println!("{}", err),
        }
    }

    println!("Subranges found: {}", subranges_found);
    println!("Overlaps found: {}", overlaps_found);

    Ok(())
}

fn convert_tuple(str_tuple: (&str, &str)) -> (i32, i32) {
    (
        str_tuple.0.parse::<i32>().unwrap(),
        str_tuple.1.parse::<i32>().unwrap(),
    )
}

fn check_subrange(first_range: (i32, i32), second_range: (i32, i32)) -> bool {
    if first_range.0 >= second_range.0 && first_range.0 <= second_range.1 {
        return true;
    } else if second_range.0 <= first_range.0 && second_range.0 >= first_range.1 {
        return true;
    }

    false
}

fn check_overlap(first_range: (i32, i32), second_range: (i32, i32)) -> bool {
    let range_max = max(first_range.0, second_range.0);
    let range_min = min(first_range.1, second_range.1);

    if range_max <= range_min {
        return true;
    }

    false
}
