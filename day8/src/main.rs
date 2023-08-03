use std::fmt::Display;

use colored::Colorize;
use itertools::Itertools;

struct Tree {
    height: usize,
    visible: bool,
}

impl Tree {
    fn new(height: usize) -> Self {
        Tree {
            height,
            visible: false,
        }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " {} ", self.height)
    }
}

struct Grid {
    width: usize,
    height: usize,
    trees: Vec<Tree>,
}

impl Grid {
    fn get_tree(&self, x: usize, y: usize) -> &Tree {
        &self.trees[x + self.width * y]
    }

    fn set_visible(&mut self, x: usize, y: usize, tallest: usize) -> usize {
        let tree = &mut self.trees[x + self.width * y];
        tree.visible = tree.height > tallest;
        tree.height
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = (0..self.height)
            .map(|j| {
                (0..self.width)
                    .map(|i| {
                        let tree = self.get_tree(i, j);

                        if tree.visible {
                            tree.to_string().green()
                        } else {
                            tree.to_string().red()
                        }
                    })
                    .join("")
            })
            .join("\n");

        write!(f, "{}", x)
    }
}

fn main() {
    let input = include_str!("example.txt");

    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    let trees = input
        .lines()
        .flat_map(|s| s.chars())
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .map(Tree::new)
        .collect::<Vec<_>>();

    let mut grid = Grid {
        width,
        height,
        trees,
    };

    for x in 0..grid.width {
        let mut tallest = 0;
        for y in 0..grid.height {
            tallest = grid.set_visible(x, y, tallest);
        }
    }

    for x in 0..grid.width {
        let mut tallest = 0;
        for y in (0..grid.height).rev() {
            tallest = grid.set_visible(x, y, tallest);
        }
    }

    for y in 0..grid.height {
        let mut tallest = 0;
        for x in 0..grid.width {
            tallest = grid.set_visible(x, y, tallest);
        }
    }

    for y in 0..grid.height {
        let mut tallest = 0;
        for x in (0..grid.width).rev() {
            tallest = grid.set_visible(x, y, tallest);
        }
    }

    println!("\n{}", grid);
    println!(
        "Visible: {}",
        grid.trees.iter().filter(|x| x.visible).count()
    )
}
