use std::fs::read_to_string;
use itertools::Itertools;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .enumerate()
        .find(|(_, window)| {
            window.len() == window
                .into_iter()
                .unique()
                .count()
        })
        .unwrap()
        .0 + 4
}

fn part_2(input: &str) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .enumerate()
        .find(|(_, window)| {
            window.len() == window
                .into_iter()
                .unique()
                .count()
        })
        .unwrap()
        .0 + 14
}

#[cfg(test)]
mod tests {
    use super::*;
    const MOCK_INPUT1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const MOCK_INPUT2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const MOCK_INPUT3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const MOCK_INPUT4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const MOCK_INPUT5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part_1() {
        assert_eq!(7, part_1(MOCK_INPUT1));
        assert_eq!(5, part_1(MOCK_INPUT2));
        assert_eq!(6, part_1(MOCK_INPUT3));
        assert_eq!(10, part_1(MOCK_INPUT4));
        assert_eq!(11, part_1(MOCK_INPUT5));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(19, part_2(MOCK_INPUT1));
        assert_eq!(23, part_2(MOCK_INPUT2));
        assert_eq!(23, part_2(MOCK_INPUT3));
        assert_eq!(29, part_2(MOCK_INPUT4));
        assert_eq!(26, part_2(MOCK_INPUT5));
    }
}
