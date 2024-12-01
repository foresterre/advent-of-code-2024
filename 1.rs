use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./1/input.txt").unwrap();
    let input = input.trim();

    let (mut left, mut right) = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("   ").unwrap();

            (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap())
        })
        .collect::<(Vec<i32>, Vec<i32>)>();

    left.sort();
    right.sort();

    let sum = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum::<u32>();

    println!("part 1: {:?}", sum);

    let counts = right
        .iter()
        .fold(HashMap::<i32, i32>::new(), |mut acc, next| {
            *acc.entry(*next).or_default() += 1;
            acc
        });

    let similarity = left
        .into_iter()
        .map(|l| l * counts.get(&l).unwrap_or(&0))
        .sum::<i32>();

    println!("part 2: {:?}", similarity);
}
