#![feature(iter_array_chunks)]
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &str) -> String {
    let (starting_stacks, rearangement_procedure) = input.split_once("\n\n").unwrap();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut setup_rows = starting_stacks.lines().rev();
    for _ in 0..setup_rows.next().unwrap().chars().filter(|c| c.is_numeric()).count()  {
        stacks.push(Vec::new())
    }
    for row in setup_rows {
        for (i, [_, id, _, _]) in row.chars().array_chunks::<4>().enumerate() {
            if id != ' ' {stacks[i].push(id)}
        }
    }

    for row in rearangement_procedure.lines() {
        let (num, remainder) = row
            .split_once("move ")
            .unwrap()
            .1
            .split_once(" from ")
            .unwrap();
        let (source, destination) = remainder
            .split_once(" to ")
            .unwrap();
        
        let (num, source, destination) = (
            num.parse::<usize>().unwrap(),
            source.parse::<usize>().unwrap() - 1,
            destination.parse::<usize>().unwrap() - 1,
        );

        for _ in 0..num {
            let val = stacks[source].pop().unwrap();
            stacks[destination].push(val);
        }
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

fn part_2(input: &str) -> String {
    let (starting_stacks, rearangement_procedure) = input.split_once("\n\n").unwrap();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut setup_rows = starting_stacks.lines().rev();
    for _ in 0..setup_rows.next().unwrap().chars().filter(|c| c.is_numeric()).count()  {
        stacks.push(Vec::new())
    }
    for row in setup_rows {
        for (i, [_, id, _, _]) in row.chars().array_chunks::<4>().enumerate() {
            if id != ' ' {stacks[i].push(id)}
        }
    }

    for row in rearangement_procedure.lines() {
        let (num, remainder) = row
            .split_once("move ")
            .unwrap()
            .1
            .split_once(" from ")
            .unwrap();
        let (source, destination) = remainder
            .split_once(" to ")
            .unwrap();
        
        let (num, source, destination) = (
            num.parse::<usize>().unwrap(),
            source.parse::<usize>().unwrap() - 1,
            destination.parse::<usize>().unwrap() - 1,
        );

        let mut val = Vec::new();
        for _ in 0..num {
            val.push(stacks[source].pop().unwrap());
        }
        val.reverse();
        stacks[destination].append(&mut val);
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const MOCK_INPUT: &str = 
"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part_1() {
        assert_eq!(String::from("CMZ"), part_1(MOCK_INPUT))
    }

    #[test]
    fn test_part_2() {
        assert_eq!(String::from("MCD"), part_2(MOCK_INPUT))
    }
}
