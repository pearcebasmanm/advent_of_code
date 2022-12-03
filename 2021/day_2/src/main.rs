fn main() {
    let input = get_input();
    part_1(&input);
    part_2(&input);
}

fn get_input() -> Vec<Command> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .map(|string| string.parse::<Command>().expect("Invalid Command"))
        .collect()
}

fn part_1(data: &[Command]) {
    let mut depth = 0;
    let mut position = 0;
    for command in data {
        match command {
            Forward(val) => position += val,
            Down(val) => depth += val,
            Up(val) => depth -= val
        }
    }
    let product = depth * position;
    println!("{product}");
}

fn part_2(data: &[Command]) {
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;
    for command in data {
        match command {
            Forward(val) => {
                position += val;
                depth += aim * val;
            },
            Down(val) => aim += val,
            Up(val) => aim -= val
        }
    }
    let product = depth * position;
    println!("{product}");
}

use Command::*;
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32)
}
impl std::str::FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((command, num)) = s.split_once(' ') else {
            return Err(InsufficientParts)
        };
        let value = num.parse().ok().ok_or(InvalidValue)?;
        Ok(match command {
            "forward" => Forward(value),
            "down" => Down(value),
            "up" => Up(value),
            _ => return Err(InvalidDirection)
        })
    }
}
use ParseCommandError::*;
#[derive(Debug)]
enum ParseCommandError {
    InsufficientParts,
    InvalidDirection,
    InvalidValue,
}