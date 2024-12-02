#![feature(iter_map_windows)]

use std::os::linux::raw::stat;

fn main() {
    let input = include_str!("2/input.txt").trim();

    let safe = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|element| element.parse::<i32>().unwrap())
                .map_windows(|&[l, r]| {
                    let diff = l.abs_diff(r);

                    Safety {
                        increasing: r > l,
                        decreasing: r < l,
                        acceptable_difference: diff >= 1 && diff <= 3,
                    }
                })
                .fold(
                    Safety {
                        increasing: true,
                        decreasing: true,
                        acceptable_difference: true,
                    },
                    |acc, next| Safety {
                        increasing: acc.increasing && next.increasing,
                        decreasing: acc.decreasing && next.decreasing,
                        acceptable_difference: acc.acceptable_difference
                            && next.acceptable_difference,
                    },
                )
        })
        .filter(|safety| safety.is_safe())
        .count();

    println!("part 1: {safe}");
}

#[derive(Debug)]
struct Safety {
    // (levels are all increasing OR
    increasing: bool,
    // levels are all decreasing) AND
    decreasing: bool,
    // (adjacent levels differ by 'at least one' and 'at most three')
    acceptable_difference: bool,
}

impl Safety {
    fn is_safe(&self) -> bool {
        (self.increasing || self.decreasing) && self.acceptable_difference
    }
}
