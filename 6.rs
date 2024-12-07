use grid::Grid;
use itertools::Itertools;
use std::cmp::PartialEq;
use std::collections::HashSet;
use std::iter;

fn main() {
    let input = include_str!("6/input.txt");
    let cols = input.lines().next().unwrap().chars().count();
    let chars = input
        .lines()
        .map(|line| line.chars())
        .flatten()
        .collect::<Vec<_>>();

    let mut grid = Grid::from_vec(chars, cols);

    let guard = grid
        .indexed_iter()
        .find_map(|((y, x), &c)| (c == '^').then_some((y, x)))
        .unwrap();

    println!("guard starts at x={}, y={}", guard.1, guard.0);

    let visited = grid.visited(guard.0, guard.1);
    println!("part 1: {}", visited.len());

    // probably it's enough to only loop over all visited above and replace these with #
    let loops = visited
        .iter()
        .map(|&(y, x)| {
            if y == guard.0 && x == guard.1 {
                return 0;
            }

            let orig = *grid.get(y, x).unwrap();
            *grid.get_mut(y, x).unwrap() = '#'; // override

            let res = grid.find_loop(guard.0, guard.1);
            *grid.get_mut(y, x).unwrap() = orig;

            res
        })
        .sum::<usize>();

    println!("part 2: {}", loops);

    // to measure speed diff:

    // let mut loops = 0_usize;
    // for y in 0..grid.rows() {
    //     for x in 0..grid.cols() {
    //         if y == guard.0 && x == guard.1 {
    //             continue;
    //         }
    //         // println!("--- iteration: {y},{x} ---");
    //
    //         let orig = *grid.get(y, x).unwrap();
    //         *grid.get_mut(y, x).unwrap() = '#'; // override
    //
    //         let res = grid.find_loop(guard.0, guard.1);
    //         *grid.get_mut(y, x).unwrap() = orig;
    //
    //         loops += res;
    //     }
    // }
}

trait Patrol {
    fn patrol(
        &self,
    ) -> impl Iterator<Item = impl Fn(usize, usize, Heading) -> Option<(usize, usize, char)>>;

    /// Patrol a given area according to the strict protocol of 1518.
    ///
    /// Returns coordinates visited during the patrol.
    fn visited(&self, start_y: usize, start_x: usize) -> HashSet<(usize, usize)>;

    fn find_loop(&self, height: usize, width: usize) -> usize;
}

impl Patrol for Grid<char> {
    fn patrol(
        &self,
    ) -> impl Iterator<Item = impl Fn(usize, usize, Heading) -> Option<(usize, usize, char)>> {
        iter::repeat(|y, x, heading: Heading| {
            if let Some((yy, xx)) = heading.step(y, x) {
                self.get(yy, xx).map(|&c| (yy, xx, c))
            } else {
                None
            }
        })
    }

    fn visited(&self, start_y: usize, start_x: usize) -> HashSet<(usize, usize)> {
        self.patrol()
            .scan((start_y, start_x, Heading::North), |state, peek| {
                let (y, x, heading) = state;

                let next_pos = peek(*y, *x, *heading);

                // .scan stops if we return a None, so in visited,
                // we stop if we go out of bounds.
                if let Some((yy, xx, c)) = next_pos {
                    // obstacle
                    if c == '#' {
                        *heading = heading.turn();
                    } else {
                        *y = yy;
                        *x = xx;
                    }

                    Some((*y, *x))
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>()
    }

    // guard loops if they revisit a coordinate with identical direction
    // or alternatively, if they visit a point 4 times, but first definition
    // is probably better to save ourselves some walking :).
    fn find_loop(&self, start_y: usize, start_x: usize) -> usize {
        self.patrol()
            .scan(
                State::new((start_y, start_x, Heading::North)),
                |state, peek| {
                    let (y, x, heading) = &mut state.guard;

                    let next_pos = peek(*y, *x, *heading);

                    let coord = (*y, *x, *heading);
                    if state.visited.contains(&coord) {
                        return Some(Status::Loop); // Detected loop
                    }

                    if let Some((yy, xx, c)) = next_pos {
                        // obstacle
                        if c == '#' {
                            *heading = heading.turn();
                        } else {
                            *y = yy;
                            *x = xx;
                        }

                        state.visited.insert(coord);
                        Some(Status::Continue) // Not a loop
                    } else {
                        Some(Status::OutOfBounds)
                    }
                },
            )
            .take_while_inclusive(|s| matches!(s, Status::Continue))
            .map(|s| s.value())
            .sum()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn step(&self, y: usize, x: usize) -> Option<(usize, usize)> {
        let x = match self {
            Heading::East => x.checked_add(1)?,
            Heading::West => x.checked_sub(1)?,
            Heading::North | Heading::South => x,
        };

        let y = match self {
            Heading::North => y.checked_sub(1)?,
            Heading::South => y.checked_add(1)?,
            Heading::East | Heading::West => y,
        };

        Some((y, x))
    }

    fn turn(&self) -> Heading {
        match self {
            Heading::North => Heading::East,
            Heading::East => Heading::South,
            Heading::South => Heading::West,
            Heading::West => Heading::North,
        }
    }
}

struct State {
    pub visited: HashSet<(usize, usize, Heading)>,
    pub guard: (usize, usize, Heading),
}

impl State {
    pub fn new(guard: (usize, usize, Heading)) -> Self {
        Self {
            visited: Default::default(),
            guard,
        }
    }
}

enum Status {
    Loop,
    OutOfBounds,
    Continue,
}

impl Status {
    fn value(self) -> usize {
        match self {
            Status::Loop => 1,
            _ => 0,
        }
    }
}
