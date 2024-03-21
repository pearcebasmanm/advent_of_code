use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> u32 {
    input.lines().map(process_line).sum()
}

// fn parse_line(line: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>)

fn process_line(line: &str) -> u32 {
    let (springs, groups) = line.split_once(' ').unwrap();
    let springs: Vec<&str> = springs
        .split('.')
        .filter(|spring| !spring.is_empty())
        .collect();
    let groups: Vec<usize> = groups
        .split(',')
        .map(|group| group.parse().unwrap())
        .collect();

    combinations(springs, &groups) as u32

    // let min_group = groups.iter().min().unwrap();

    // springs
    //     .iter()
    //     .map(|spring| 0..(spring.len() / min_group))
    //     .multi_cartesian_product()
    //     .filter(|amounts| {
    //         let mut groups = groups.iter();
    //         springs.iter().map(|spring|)
    //     })
    //     .count() as u32
}

fn combinations(mut springs: Vec<&str>, groups: &[usize]) -> usize {
    if groups.len() == 1 {
        let group = groups[0];
        let forced_placements: Vec<_> = springs
            .iter()
            .filter(|spring| spring.contains('#'))
            .collect();
        return match forced_placements[..] {
            [] => springs
                .into_iter()
                .map(|spring| (spring.len() + 1).saturating_sub(group))
                .sum(),
            [spring] => {
                if spring.len() < group {
                    return 0;
                }
                let forced_indexes: Vec<_> = spring
                    .chars()
                    .enumerate()
                    .filter(|&(_, c)| c == '#')
                    .map(|(i, _)| i)
                    .collect();
                let forced_span =
                    forced_indexes.iter().max().unwrap() - forced_indexes.iter().min().unwrap();
                if forced_span > group {
                    return 0;
                }
                x
            }
            [_, _, ..] => 0,
        };
    }
    let mut product = 0;

    let left = 0;
    let right = springs.len() - 1;

    if let (Some(spring), Some(&group)) = (springs.first(), groups.first()) {
        if group > spring.len() {
            springs.remove(0);
        }
    }

    product
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    const MOCK_SOLUTION: u32 = 21;

    #[test]
    fn test() {
        assert_eq!(part1(MOCK_INPUT), MOCK_SOLUTION);
    }
}
