use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;

fn main() {
    let input = include_str!("5/input.txt");

    let mut lines = input.lines();

    let rules = lines
        .take_while_ref(|&line| !line.is_empty())
        .map(|line| line.split_once('|').unwrap())
        .collect::<HashSet<_>>();

    let manuals = lines.skip(1);

    let correct = manuals
        .clone()
        .filter_map(|manual| {
            let page = manual.split(',').collect::<Vec<_>>();

            page.iter()
                .is_sorted_by(|&l, &r| rules.contains(&(l, r)))
                .then_some(page[page.len() / 2].parse::<u32>().unwrap())
        })
        .inspect(|middle| {
            dbg!(middle);
        })
        .sum::<u32>();

    println!("part 1: {}", correct);

    let incorrect = manuals
        .filter_map(|manual| {
            let page = manual.split(',').collect::<Vec<_>>();

            let sorted = page
                .iter()
                .sorted_by(|&&l, &&r| {
                    if rules.contains(&(l, r)) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .copied()
                .collect::<Vec<_>>();

            (page != sorted).then_some(sorted[sorted.len() / 2].parse::<u32>().unwrap())
        })
        .inspect(|middle| {
            dbg!(middle);
        })
        .sum::<u32>();

    println!("part 2: {}", incorrect);
}
