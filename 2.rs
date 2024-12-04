use itertools::Itertools;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("2/input.txt").trim();

    let safe = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|element| element.parse::<i32>().unwrap())
                .tuple_windows()
                .fold(Safety::default(), |acc, (l, r)| acc.test(l, r))
        })
        .filter(|safety| safety.is_safe())
        .count();

    println!("part 1: {safe}");
}

fn part2() {
    let input = include_str!("2/input.txt").trim();

    let safe = input
        .lines()
        .map(|line| {
            let report = line
                .split_ascii_whitespace()
                .map(|element| element.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            let check = report
                .iter()
                .tuple_windows()
                .fold(Safety::default(), |acc, (&l, &r)| acc.test(l, r));

            if check.is_safe() {
                return true;
            }

            (0..report.len())
                .map(|i| {
                    report
                        .iter()
                        .enumerate()
                        .filter_map(|(j, e)| if j != i { Some(e) } else { None }) // skip nth
                        .tuple_windows()
                        .fold(Safety::default(), |acc, (&l, &r)| acc.test(l, r))
                })
                .any(|s| s.is_safe())
        })
        .filter(|&s| s)
        .count();

    println!("part 2: {safe}");
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

    fn test(&self, l: i32, r: i32) -> Self {
        let diff = l.abs_diff(r);

        Safety {
            increasing: self.increasing && r > l,
            decreasing: self.decreasing && r < l,
            acceptable_difference: self.acceptable_difference && diff >= 1 && diff <= 3,
        }
    }
}

impl Default for Safety {
    fn default() -> Self {
        Self {
            increasing: true,
            decreasing: true,
            acceptable_difference: true,
        }
    }
}
