use regex::Regex;
use std::sync::LazyLock;

static MUL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

fn summed_mul(input: &'static str) -> u32 {
    MUL.captures_iter(input)
        .map(|caps| {
            let l = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let r = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();

            l * r
        })
        .sum::<u32>()
}

fn main() {
    let input = include_str!("3/input.txt");
    let sum = summed_mul(input);

    println!("part 1: {}", sum);

    let switch_regex = Regex::new(r"(do\(\)|don't\(\))").unwrap();

    let mut state = switch_regex
        .captures_iter(input)
        .fold(State::default(), |mut acc, caps| {
            let capture = caps.get(1).unwrap();

            match capture.as_str() {
                "do()" if acc.enabled_from.is_none() => {
                    // enable
                    acc.enabled_from = Some(capture.end());
                }
                "don't()" => {
                    if let Some(from) = acc.enabled_from {
                        // compute multiplicative sum for current input range
                        let subset = &input[from..capture.start()];
                        acc.sum += summed_mul(subset);

                        // disable
                        acc.enabled_from = None;
                    }
                }
                _ => {}
            };

            acc
        });

    // Add final open subset
    if let Some(from) = state.enabled_from {
        let subset = &input[from..];
        state.sum += summed_mul(subset);
    }

    println!("part 2: {}", state.sum);
}

#[derive(Debug)]
struct State {
    enabled_from: Option<usize>,
    sum: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            enabled_from: Some(0),
            sum: 0,
        }
    }
}
