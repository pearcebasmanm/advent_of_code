use std::fs::read_to_string;

fn main() {        
    let input = read_to_string("input.txt").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(' ').unwrap();
            let mut score = 0;
            score += match right {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => panic!()
            };
            score += match (left, right) {
                ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
                ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
                ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
                _ => panic!()
            };
            score
        })
        .sum::<i32>()
}

fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(' ').unwrap();
            let mut score = 0;
            score += match right {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => panic!()
            };
            score += match (left, right) {
                ("A", "Y") | ("B", "X") | ("C", "Z") => 1,
                ("A", "Z") | ("B", "Y") | ("C", "X") => 2,
                ("A", "X") | ("B", "Z") | ("C", "Y") => 3,
                _ => panic!()
            };
            score
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const MOCK_INPUT: &str = 
"A Y
B X
C Z";

    #[test]
    fn test_part_1() {
        assert_eq!(15, part_1(MOCK_INPUT))
    }

    #[test]
    fn test_part_2() {
        assert_eq!(12, part_2(MOCK_INPUT))
    }
}