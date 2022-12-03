use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut sum_priorities: i32 = 0;
    let mut sum_badges: i32 = 0;
    let mut i = 0;
    let mut elves_group: Vec<String> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let halfway = line.len() / 2;

                let compartment_one = &line[0..halfway];
                let compartment_two = &line[halfway..line.len()];

                let similarity = find_similarity(compartment_one, compartment_two, false);

                match similarity {
                    Some(prio) => sum_priorities += get_value(&prio),
                    None => panic!("A similarity should have been found."),
                }

                if i % 3 == 0 && i > 0 {
                    let badge = find_badge(&elves_group[0], &elves_group[1], &elves_group[2]);

                    match badge {
                        Some(badge) => sum_badges += get_value(&badge),
                        None => panic!("A similarity should have been found."),
                    }
                    elves_group.clear();
                }
                elves_group.push(line);
                i += 1;
            }
            Err(err) => println!("{}", err),
        }
    }

    // get the last three in
    let badge = find_badge(&elves_group[0], &elves_group[1], &elves_group[2]);

    match badge {
        Some(badge) => sum_badges += get_value(&badge),
        None => panic!("A similarity should have been found."),
    }

    println!("Sum of priorities: {}", sum_priorities);
    println!("Sum of badges: {}", sum_badges);

    Ok(())
}

fn find_badge(group_one: &String, group_two: &String, group_three: &String) -> Option<String> {
    if let Some(first_similarity) = find_similarity(&group_one, &group_two, true) {
        if let Some(final_similarity) = find_similarity(&first_similarity, &group_three, false) {
            return Some(final_similarity);
        }
    }

    return None;
}

fn get_value(string: &String) -> i32 {
    let converted_char = string.chars().next().expect("string is empty");

    match converted_char.is_uppercase() {
        true => converted_char as i32 - 38,
        false => converted_char as i32 - 96,
    }
}

fn find_similarity(str_one: &str, str_two: &str, multiple: bool) -> Option<String> {
    if str_one.chars().count() == 0 || str_two.chars().count() == 0 {
        return None;
    }

    let set: HashSet<char> = str_one.chars().collect();

    let mut similarity = String::new();

    for c in str_two.chars() {
        if set.contains(&c) {
            if multiple {
                similarity.push(c);
            } else {
                similarity = c.to_string();
            }
        }
    }

    Some(similarity)
}
