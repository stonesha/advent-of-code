use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// A = Rock = 1, B = Paper = 2, C = Scissors = 3
// presumably, X = Rock = 1, Y = Paper = 2, Z = Scissors = 3
// loss = 0, draw = 3, win = 6
fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut answer_one = 0;
    let mut answer_two = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let split = line.split(" ").collect::<Vec<&str>>();
                let opponents_choice = split[0];
                let player_choice = split[1];

                answer_one += match opponents_choice {
                    "A" => handle_opponent_rock(player_choice),
                    "B" => handle_opponent_paper(player_choice),
                    "C" => handle_opponent_scissors(player_choice),
                    &_ => 0,
                };

                answer_two += match player_choice {
                    "X" => handle_player_loss(opponents_choice),
                    "Y" => handle_player_tie(opponents_choice),
                    "Z" => handle_player_win(opponents_choice),
                    &_ => 0,
                };
            }
            Err(err) => println!("{}", err),
        }
    }

    println!("The answer for part one is: {}", answer_one);
    println!("The answer for part two is: {}", answer_two);

    Ok(())
}

fn handle_player_loss(opponents_choice: &str) -> i32 {
    match opponents_choice {
        "A" => 3 + 0,
        "B" => 1 + 0,
        "C" => 2 + 0,
        &_ => 0,
    }
}

fn handle_player_tie(opponents_choice: &str) -> i32 {
    match opponents_choice {
        "A" => 1 + 3,
        "B" => 2 + 3,
        "C" => 3 + 3,
        &_ => 0,
    }
}

fn handle_player_win(opponents_choice: &str) -> i32 {
    match opponents_choice {
        "A" => 2 + 6,
        "B" => 3 + 6,
        "C" => 1 + 6,
        &_ => 0,
    }
}

fn handle_opponent_rock(player_choice: &str) -> i32 {
    match player_choice {
        "X" => 1 + 3,
        "Y" => 2 + 6,
        "Z" => 3 + 0,
        &_ => 0,
    }
}

fn handle_opponent_paper(player_choice: &str) -> i32 {
    match player_choice {
        "X" => 1 + 0,
        "Y" => 2 + 3,
        "Z" => 3 + 6,
        &_ => 0,
    }
}

fn handle_opponent_scissors(player_choice: &str) -> i32 {
    match player_choice {
        "X" => 1 + 6,
        "Y" => 2 + 0,
        "Z" => 3 + 3,
        &_ => 0,
    }
}
