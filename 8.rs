use grid::Grid;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::{Formatter, Write};
use std::{fmt, ops};

fn main() {
    let input = include_str!("8/input.txt");
    let cols = input.lines().next().unwrap().chars().count();

    let chars = input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<Vec<char>>();

    let grid = Grid::from_vec(chars.clone(), cols);

    let antennas = grid.indexed_iter().fold(
        HashMap::<char, Vec<Node>>::default(),
        |mut map, ((y, x), &c)| {
            if c != '.' {
                map.entry(c).or_default().push((y, x).into());
            }
            map
        },
    );

    let set = antennas
        .values()
        .fold(HashSet::<Node>::default(), |mut acc, nodes| {
            let permutations = nodes.iter().permutations(2);

            for permutation in permutations {
                let (&a, &b) = (permutation[0], permutation[1]);

                let antinode = a + (a - b);
                if let Some(_node) = grid.get(antinode.y(), antinode.x()) {
                    acc.insert(antinode);
                }

                let antinode = b + (b - a);
                if let Some(_node) = grid.get(antinode.y(), antinode.x()) {
                    acc.insert(antinode);
                }
            }

            acc
        });

    println!("part 1: {}", set.len());

    let mut set = antennas.values().filter(|nodes| nodes.len() >= 2).fold(
        HashSet::<Node>::default(),
        |mut acc, nodes| {
            let permutations = nodes.iter().permutations(2);

            for permutation in permutations {
                let (&a, &b) = (permutation[0], permutation[1]);

                let diff = a - b;
                let mut antinode = a + diff;
                while grid.get(antinode.y(), antinode.x()).is_some() {
                    acc.insert(antinode);
                    antinode = antinode + diff;
                }

                let diff = b - a;
                let mut antinode = b + diff;
                while grid.get(antinode.y(), antinode.x()).is_some() {
                    acc.insert(antinode);
                    antinode = antinode + diff;
                }
            }

            acc
        },
    );

    let self_antinodes = antennas
        .values()
        .filter(|nodes| nodes.len() >= 2)
        .flatten()
        .copied()
        .collect::<HashSet<Node>>();

    set.extend(self_antinodes);

    println!("{}", AntinodePrinter::new(&grid, &set));
    println!("part 2: {}", set.len());
}

// y by x, since our grid is row major
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Node(isize, isize);

impl From<(usize, usize)> for Node {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0 as isize, value.1 as isize)
    }
}

impl ops::Add<Node> for Node {
    type Output = Node;

    fn add(self, rhs: Node) -> Self::Output {
        let y = self.0 + rhs.0;
        let x = self.1 + rhs.1;

        Node(y, x)
    }
}

impl ops::Sub<Node> for Node {
    type Output = Node;

    fn sub(self, rhs: Node) -> Self::Output {
        let y = self.0 - rhs.0;
        let x = self.1 - rhs.1;

        Node(y, x)
    }
}

impl Node {
    pub fn x(&self) -> isize {
        self.1
    }

    pub fn y(&self) -> isize {
        self.0
    }
}

struct AntinodePrinter<'g, 'a> {
    grid: &'g Grid<char>,
    antinodes: &'a HashSet<Node>,
}

impl<'g, 'a> AntinodePrinter<'g, 'a> {
    pub fn new(grid: &'g Grid<char>, antinodes: &'a HashSet<Node>) -> Self {
        Self { grid, antinodes }
    }
}

impl<'g, 'a> fmt::Display for AntinodePrinter<'g, 'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for ((y, x), &c) in self.grid.indexed_iter() {
            if x == 0 {
                f.write_char('\n')?
            }

            if self.antinodes.contains(&(y, x).into()) {
                f.write_char('#')?;
            } else {
                f.write_char(c)?;
            }
        }

        Ok(())
    }
}
