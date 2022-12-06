use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt")
        .chars()
        .enumerate()
        .collect::<Vec<_>>();

    let first = input
        .windows(14)
        .find(|w| w.iter().permutations(2).all(|a| a[0].1 != a[1].1))
        .unwrap()
        .last()
        .unwrap()
        .0
        + 1;

    dbg!(first);
}
