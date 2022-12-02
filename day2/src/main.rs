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

fn outcome(opponent: Shape, response: Shape) -> Outcome {
    if opponent == response {
        Outcome::Draw
    } else if opponent == Shape::Rock {
        if response == Shape::Scissors {
            Outcome::Loss
        } else {
            Outcome::Win
        }
    } else if opponent == Shape::Paper {
        if response == Shape::Scissors {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    } else if response == Shape::Rock {
        Outcome::Win
    } else {
        Outcome::Loss
    }
}

fn score(opponent: Shape, response: Shape) -> usize {
    let outcome_score = match outcome(opponent, response) {
        Outcome::Win => 6,
        Outcome::Loss => 0,
        Outcome::Draw => 3,
    };

    let shape_score = match response {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

    outcome_score + shape_score
}

fn parse(c: &str) -> Shape {
    if c == "A" || c == "X" {
        Shape::Rock
    } else if c == "B" || c == "Y" {
        Shape::Paper
    } else if c == "C" || c == "Z" {
        Shape::Scissors
    } else {
        panic!("Unknown shape {}", c)
    }
}

fn main() {
    let score = include_str!("input.txt")
        .lines()
        .map(|l| l.split(' ').map(parse).collect::<Vec<_>>())
        .map(|g| score(g[0], g[1]))
        .sum::<usize>();

    println!("{score}");
}
