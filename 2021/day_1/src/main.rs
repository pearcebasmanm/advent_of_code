fn main() {
    // let mock_input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let input = get_input();
    part_1(&input);
    part_2(&input);
}

fn get_input() -> Vec<u32> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .map(|val| val.parse().unwrap())
        .collect()
}

fn part_1(data: &[u32]) {
    let increases = data
        .windows(2)
        .filter(|win| win[0] < win[1])
        .count();
    println!("{increases}");
}
fn part_2(data: &[u32]) {
    let increases = data
        .windows(3)
        .map(|win| win[0] + win[1] + win[2])
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|win| win[0] < win[1])
        .count();
    println!("{increases}");
}
