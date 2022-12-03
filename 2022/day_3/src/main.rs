#![feature(iter_array_chunks)]
use std::fs::read_to_string;

fn main() {        
    let input = read_to_string("input.txt").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|bag| {
            let (first_compartment, second_compartment) = bag.split_at(bag.len() / 2);
            let common_character = first_compartment
                .chars()
                .find(|&character| second_compartment.contains(character))
                .unwrap();
            common_character as i32 - if common_character.is_uppercase() {38} else {96}
        })
        .sum()
}

fn part_2(input: &str) -> i32 {
    input
        .lines()
        .array_chunks()
        .map(|[first_bag, second_bag, third_bag]| {
            let common_character = first_bag
                .chars()
                .find(|&character| second_bag.contains(character) && third_bag.contains(character))
                .unwrap();
            common_character as i32 - if common_character.is_uppercase() {38} else {96}
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const MOCK_INPUT: &str = 
"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_1() {
        assert_eq!(157, part_1(MOCK_INPUT))
    }

    #[test]
    fn test_part_2() {
        assert_eq!(70, part_2(MOCK_INPUT))
    }
}