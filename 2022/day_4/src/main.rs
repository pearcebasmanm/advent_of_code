use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|pair| {
            let [[left_start, left_end], [right_start, right_end]]: [[i32; 2]; 2] = pair
                .split(',')
                .map(|side| {
                    side.split('-')
                        .map(|val| val.parse().unwrap())
                        .collect::<Vec<i32>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<[i32; 2]>>()
                .try_into()
                .unwrap();

            (left_start <= right_start && right_end <= left_end)
                || (right_start <= left_start && left_end <= right_end)
        })
        .count()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|pair| {
            let [[left_start, left_end], [right_start, right_end]]: [[i32; 2]; 2] = pair
                .split(',')
                .map(|side| {
                    side.split('-')
                        .map(|val| val.parse().unwrap())
                        .collect::<Vec<i32>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<[i32; 2]>>()
                .try_into()
                .unwrap();

            (left_start <= right_start && right_start <= left_end)
                || (right_start <= left_start && left_start <= right_end)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const MOCK_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part_1() {
        assert_eq!(2, part_1(MOCK_INPUT))
    }

    #[test]
    fn test_part_2() {
        assert_eq!(4, part_2(MOCK_INPUT))
    }
}
