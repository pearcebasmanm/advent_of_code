use std::fs::read_to_string;

fn main() {        
    let input = read_to_string("input.txt").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|elf| {
            elf
                .lines()
                .map(|string| string.parse::<i32>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

fn part_2(input: &str) -> i32 {
    let mut elves: Vec<i32> = input
        .split("\n\n")
        .map(|elf| 
            elf
                .lines()
                .map(|string| string.parse::<i32>().unwrap())
                .sum()
        )
        .collect();
    elves.sort();
    elves.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const MOCK_INPUT: &str = 
"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part_1() {
        assert_eq!(24000, part_1(MOCK_INPUT))
    }

    #[test]
    fn test_part_2() {
        assert_eq!(45000, part_2(MOCK_INPUT))
    }
}