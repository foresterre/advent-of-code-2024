use grid::Grid;

fn main() {
    let input = include_str!("4/input.txt");
    let cols = input.lines().next().unwrap().chars().count();
    let chars = input
        .lines()
        .map(|line| line.chars())
        .flatten()
        .collect::<Vec<_>>();

    let mut grid = Grid::from_vec(chars, cols);

    let part1 = word_search_xmas_line(&mut grid);
    println!("part 1: {part1}");

    let part2 = word_search_masmas(&grid);
    println!("part 2: {part2}");
}

fn word_search_xmas_line(grid: &mut Grid<char>) -> usize {
    fn is_xmas(xmas: &[char; 4]) -> bool {
        *xmas == ['X', 'M', 'A', 'S'] || *xmas == ['S', 'A', 'M', 'X']
    }

    grid.indexed_iter()
        .map(|((j, i), _c)| {
            // offset grid relative to the line, to prevent underflow
            [
                // horizontal
                [
                    grid.get(j, i).copied().unwrap_or_default(),
                    grid.get(j, i + 1).copied().unwrap_or_default(),
                    grid.get(j, i + 2).copied().unwrap_or_default(),
                    grid.get(j, i + 3).copied().unwrap_or_default(),
                ],
                // vertical
                [
                    grid.get(j, i).copied().unwrap_or_default(),
                    grid.get(j + 1, i).copied().unwrap_or_default(),
                    grid.get(j + 2, i).copied().unwrap_or_default(),
                    grid.get(j + 3, i).copied().unwrap_or_default(),
                ],
                // diagonal variant 1: top left - bottom right
                [
                    grid.get(j, i).copied().unwrap_or_default(),
                    grid.get(j + 1, i + 1).copied().unwrap_or_default(),
                    grid.get(j + 2, i + 2).copied().unwrap_or_default(),
                    grid.get(j + 3, i + 3).copied().unwrap_or_default(),
                ],
                // diagonal variant 2: bottom left - top right (checked sub to prevent underflow)
                [
                    i.checked_sub(3)
                        .and_then(|v| grid.get(j + 3, v))
                        .copied()
                        .unwrap_or_default(),
                    i.checked_sub(2)
                        .and_then(|v| grid.get(j + 2, v))
                        .copied()
                        .unwrap_or_default(),
                    i.checked_sub(1)
                        .and_then(|v| grid.get(j + 1, v))
                        .copied()
                        .unwrap_or_default(),
                    grid.get(j, i).copied().unwrap_or_default(),
                ],
            ]
        })
        .flatten()
        .filter(is_xmas)
        .count()
}

fn word_search_masmas(grid: &Grid<char>) -> usize {
    fn is_x(xmas: &[char; 5]) -> bool {
        *xmas == ['M', 'M', 'A', 'S', 'S']
            || *xmas == ['M', 'S', 'A', 'M', 'S']
            || *xmas == ['S', 'M', 'A', 'S', 'M']
            || *xmas == ['S', 'S', 'A', 'M', 'M']
    }

    grid.indexed_iter()
        .map(|((j, i), _c)| {
            // offset grid relative to the "X", to prevent underflow
            [
                // top left
                grid.get(j, i).copied().unwrap_or_default(),
                // top right
                grid.get(j, i + 2).copied().unwrap_or_default(),
                // center
                grid.get(j + 1, i + 1).copied().unwrap_or_default(),
                // bottom left
                grid.get(j + 2, i).copied().unwrap_or_default(),
                // bottom right
                grid.get(j + 2, i + 2).copied().unwrap_or_default(),
            ]
        })
        .filter(is_x)
        .count()
}
