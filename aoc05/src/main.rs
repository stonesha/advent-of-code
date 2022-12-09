use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut stacks_one: Vec<String> = vec![
        "GFVHPS".to_string(),
        "GJFBVDZM".to_string(),
        "GMLJN".to_string(),
        "NGZVDWP".to_string(),
        "VRCB".to_string(),
        "VRSMPWLZ".to_string(),
        "THP".to_string(),
        "QRSNCHZV".to_string(),
        "FLGPVQJ".to_string(),
    ];

    let mut stacks_two = stacks_one.clone();

    let mut move_instructions = false;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if move_instructions {
                    println!("move_instructions: {}", line);
                    let captures = extract_instructions(&line);

                    let mut index_one: usize = FromStr::from_str(&captures["one"]).unwrap();
                    let mut index_two: usize = FromStr::from_str(&captures["two"]).unwrap();

                    index_one -= 1;
                    index_two -= 1;

                    let quantity: usize = FromStr::from_str(&captures["quantity"]).unwrap();

                    let (stack_one, stack_two) = perform_instructions(
                        stacks_one[index_one].to_string(),
                        stacks_one[index_two].to_string(),
                        quantity,
                    );

                    stacks_one[index_one] = stack_one;
                    stacks_one[index_two] = stack_two;

                    let (stack_one, stack_two) = perform_instructions_for_cratemover_9001(
                        stacks_two[index_one].to_string(),
                        stacks_two[index_two].to_string(),
                        quantity,
                    );

                    stacks_two[index_one] = stack_one;
                    stacks_two[index_two] = stack_two;
                }

                if line == "" {
                    move_instructions = true;
                }
            }
            Err(err) => println!("{}", err),
        }
    }

    let topmost_letters_one: String = stacks_one
        .iter()
        .map(|x| x.chars().last().unwrap())
        .collect::<String>();
    println!("Topmost crates are: {}", topmost_letters_one);

    let topmost_letters_two: String = stacks_two
        .iter()
        .map(|x| x.chars().last().unwrap())
        .collect::<String>();
    println!(
        "Topmost crates for CrateMover 9001 are: {}",
        topmost_letters_two
    );

    Ok(())
}

fn perform_instructions_for_cratemover_9001(
    mut stack_one: String,
    mut stack_two: String,
    quantity: usize,
) -> (String, String) {
    println!(
        "before change: stack one: {}, stack_two: {}, quantity: {}",
        stack_one, stack_two, quantity
    );
    let new_quantity = stack_one.len() - quantity;
    let n_top_crates: String = stack_one[new_quantity..].to_string();

    stack_two.push_str(&n_top_crates);
    stack_one = stack_one[..new_quantity].to_string();

    println!(
        "after change: stack one: {}, stack_two: {}",
        stack_one, stack_two
    );
    (stack_one, stack_two)
}

fn perform_instructions(
    mut stack_one: String,
    mut stack_two: String,
    quantity: usize,
) -> (String, String) {
    for _ in 0..quantity {
        let popped_letter = stack_one.pop().unwrap();

        stack_two.push(popped_letter)
    }

    (stack_one, stack_two)
}

fn extract_instructions(input: &str) -> Captures {
    lazy_static! {
        static ref INSTRUCTIONS_REGEX: Regex =
            Regex::new(r"move (?P<quantity>\d+) from (?P<one>\d{1}) to (?P<two>\d{1})").unwrap();
    }

    let caps = INSTRUCTIONS_REGEX.captures(input).unwrap();

    caps
}
