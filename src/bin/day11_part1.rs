use std::cmp::{max, min};

use itertools::Itertools;

fn main() {
    let galaxies: Vec<(usize, usize)> = std::fs::read_to_string("input/day11")
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(line_index, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(column, c)| (c == '#').then_some((line_index, column)))
        })
        .collect();

    fn axis_positions(
        galaxies: &[(usize, usize)],
        f: impl Fn(&(usize, usize)) -> usize,
    ) -> Vec<usize> {
        let mut result: Vec<_> = galaxies.iter().map(f).collect();
        result.sort();
        result.dedup();
        result
    }

    let rows = axis_positions(&galaxies, |&(row, _col)| row);
    let columns = axis_positions(&galaxies, |&(_row, col)| col);

    fn distance(galaxy_positions: &[usize], a: usize, b: usize) -> usize {
        if a == b {
            return 0;
        }
        let min = min(a, b);
        let max = max(a, b);

        let lower_bound = |x| match galaxy_positions.binary_search(&x) {
            Ok(index) => index,
            Err(index) => index,
        };

        let galaxy_positions_between = lower_bound(max) - lower_bound(min + 1);
        let empty_positions_between = max - min - 1 - galaxy_positions_between;

        max - min + empty_positions_between
    }

    let mut answer = 0;
    for (a, b) in galaxies.iter().tuple_combinations() {
        let row_distance = distance(&rows, a.0, b.0);
        let col_distance = distance(&columns, a.1, b.1);
        let distance = row_distance + col_distance;
        answer += distance;
    }
    println!("{answer}");
}
