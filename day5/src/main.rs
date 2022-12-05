use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{char, digit1},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn main() {
    let input = include_str!("input.txt");
    let (initial, instructions) = input.split("\n\n").collect_tuple().unwrap();

    println!("{initial}");

    let mut parsed_initial = get_initial_setup(initial);

    let parsed_instructions = instructions
        .lines()
        .map(|l| parse_instruction(l).unwrap().1)
        .collect::<Vec<_>>();

    for instruction in parsed_instructions {
        let mut stack = Vec::new();

        for _ in 0..instruction.count {
            if let Some(c) = parsed_initial[instruction.from - 1].pop() {
                stack.push(c)
            }
        }

        stack.reverse();
        for c in stack {
            parsed_initial[instruction.to - 1].push(c)
        }
    }

    for tower in parsed_initial {
        print!("{}", tower.last().unwrap())
    }
    println!();
    
}

// Crate parsing

fn get_initial_setup(initial: &str) -> Vec<Vec<&str>> {
    let mut setup = initial.lines().rev();
    setup.next();

    transpose(setup.map(|l| parse_line(l).unwrap().1).collect::<Vec<_>>())
        .into_iter()
        .map(|l| l.into_iter().flatten().collect())
        .collect()
}

fn parse_line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, crates) = separated_list0(tag(" "), parse_crate)(input)?;

    Ok((input, crates))
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let crate_parser = delimited(char('['), take(1usize), char(']'));
    let empty = tag("   ");

    let (input, c) = alt((crate_parser, empty))(input)?;

    if c == "   " {
        Ok((input, None))
    } else {
        Ok((input, Some(c)))
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

// instruction parsing

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, count) = preceded(tag("move "), digit1)(input)?;

    let (input, (from, to)) =
        preceded(tag(" from "), separated_pair(digit1, tag(" to "), digit1))(input)?;

    Ok((
        input,
        Instruction {
            count: count.parse().unwrap(),
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
        },
    ))
}
