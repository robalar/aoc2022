use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let largest = input
        .split(|x| x.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .sorted_by_key(|&v| u64::MAX - v)
        .take(3)
        .sum::<u64>();

    println!("{largest:?}");
}
