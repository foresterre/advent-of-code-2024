fn main() {
    let input = include_str!("7/input.txt");

    let (p1, p2) = input
        .lines()
        .map(|line| {
            let (target, operands) = line.split_once(':').unwrap();
            let target = target.parse::<usize>().unwrap();

            let operands = operands
                .trim_start()
                .split(' ')
                .map(|operand| operand.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let p1 = eval(target, operands[0], &operands[1..], false);
            let p2 = eval(target, operands[0], &operands[1..], true);

            (if p1 { target } else { 0 }, if p2 { target } else { 0 })
        })
        .reduce(|(acc1, acc2), (val1, val2)| (acc1 + val1, acc2 + val2))
        .unwrap();

    println!("part 1: {p1}");
    println!("part 2: {p2}");
}

fn eval(target: usize, acc: usize, operands: &[usize], enable_concat: bool) -> bool {
    if operands.is_empty() {
        return acc == target;
    }

    eval(target, acc + operands[0], &operands[1..], enable_concat)
        || eval(target, acc * operands[0], &operands[1..], enable_concat)
        || (enable_concat && eval(target, concat(acc, operands[0]), &operands[1..], true))
}

fn concat(l: usize, r: usize) -> usize {
    l * 10_usize.pow(r.ilog(10) + 1) + r
}
