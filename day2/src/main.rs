#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

fn desired_shape(opponent: Shape, desired: Outcome) -> Shape {
    match desired {
        Outcome::Win => match opponent {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
        Outcome::Loss => match opponent {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
        Outcome::Draw => opponent,
    }
}

fn score(opponent: Shape, desired: Outcome) -> usize {
    let outcome_score = match desired {
        Outcome::Win => 6,
        Outcome::Loss => 0,
        Outcome::Draw => 3,
    };

    let shape_score = match desired_shape(opponent, desired) {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

    outcome_score + shape_score
}

fn parse_shape(s: &str) -> Shape {
    match s {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Unknown shape"),
    }
}

fn parse_desired(s: &str) -> Outcome {
    match s {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Unknown outcome"),
    }
}

fn main() {
    let score = include_str!("input.txt")
        .lines()
        .map(|l| l.split(' ').collect::<Vec<_>>())
        .map(|g| score(parse_shape(g[0]), parse_desired(g[1])))
        .sum::<usize>();

    println!("{score}");
}
