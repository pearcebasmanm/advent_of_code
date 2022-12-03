use std::fs::read_to_string;

fn main() {
    // let mock_input = ["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"];
    let input = get_input();
    part_1(&input);
}

fn get_input() -> Vec<[bool;12]> {
    read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .map(|string| string
            .chars()
            .map(|char| match char {
                '1' => true,
                '0' => false,
                _ => panic!("Invalid binary digit")
            })
            .collect::<Vec<bool>>()
            .try_into()
            .unwrap()
        )
        .collect()
}

fn part_1(data: &[[bool;12]]) {
    let mut column_sums = [0;12];
    for row in data {
        for (i, &bit) in row.iter().enumerate() {
            column_sums[i] += bit as u32;
        }
    }
    let most_common = column_sums.map(|sum| (sum > data.len() as u32 / 2));

    let gamma = most_common
        .iter()
        .fold(0, |result, &bit| (result << 1) ^ bit as u32);

    let epsilon = most_common
        .map(|bool| !bool)
        .iter()
        .fold(0, |result, &bit| (result << 1) ^ bit as u32);

    let product = gamma * epsilon;
    println!("{product}");
}

fn part_2(data: &[[bool;12]]) {
    let mut columns: [Vec<bool>;12] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    for row in data {
        for (i, &bit) in row.iter().enumerate() {
            columns[i].push(bit);
        }
    }
    let most_common = columns
        .map(|column| column.iter().map(|&bit| bit as u32).sum::<u32>())
        .map(|sum| (sum > data.len() as u32 / 2));
    
    let mut generator_rows: Vec<[bool;12]> = data.into();

    for i in 0..12 {
        generator_rows = generator_rows
            .iter()
            .filter(|row| generator_rows.len() == 1 || row[i] == most_common[i])
            .copied()
            .collect::<Vec<[bool;12]>>();
    }

    let generator_rating = columns
        .map(|column| column
            .iter()
            .filter(|&&bit| bit == most_common[0])
            .copied()
            .collect::<Vec<bool>>()
        );

    let scrubber_rating = 0;
    let product = generator_rating * scrubber_rating;
    println!("{product}");
}