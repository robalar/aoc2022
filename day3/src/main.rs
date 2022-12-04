use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let lines = include_str!("input.txt")
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let sets = chunk
                .map(|l| l.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();

            let ab = sets[0]
                .intersection(&sets[1])
                .copied()
                .collect::<HashSet<_>>();
            let common = ab.intersection(&sets[2]).copied();

            common.map(get_priority).sum::<usize>()
        })
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
