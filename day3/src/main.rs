use std::collections::HashSet;

fn main() {
    let lines = include_str!("input.txt")
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|l| {
            let left: HashSet<char> = l.0.chars().collect();
            let right: HashSet<char> = l.1.chars().collect();

            left.intersection(&right).copied().collect::<HashSet<_>>()
        })
        .map(|s| s.into_iter().map(get_priority).sum::<usize>())
        .sum::<usize>();

    dbg!(lines);
}

fn get_priority(c: char) -> usize {
    let code = c as usize;
    match code {
        97..=122 => code - 96,
        65..=90 => code - 64 + 26,
        _ => panic!("Unknown priority for {}={}", c, code),
    }
}
