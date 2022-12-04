use std::ops::RangeInclusive;

fn get_range(s: &str) -> RangeInclusive<usize> {
    let mut bounds = s.split('-');
    let (Some(lower), Some(upper), None) = (bounds.next(), bounds.next(), bounds.next()) else {
        panic!("Expected <int>-<int>, got {s:?}");
    };

    lower.parse().unwrap()..=upper.parse().unwrap()
}

fn main() {
    let input = include_str!("input.txt").lines().map(|line| {
        let mut ranges = line.split(',');
        let (Some(first), Some(second), None) = (ranges.next(), ranges.next(), ranges.next()) else {
            panic!("Expected <range>,<range> got {line:?}");
        };

        (get_range(first), get_range(second))
    }).filter(|(x, y)| x.contains(y.start()) || x.contains(y.end()) || y.contains(x.start()) || y.contains(x.end())).count();

    dbg!(input);
}
