use itertools::Itertools;

fn main() {
    let input: Vec<Vec<i64>> = std::fs::read_to_string("input/day9")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    fn extrapolate(a: &[i64]) -> i64 {
        if let Ok(value) = a.iter().copied().all_equal_value() {
            return value;
        }
        let deltas: Vec<i64> = a.iter().tuple_windows().map(|(a, b)| b - a).collect();
        let prev_delta = extrapolate(&deltas);
        *a.first().unwrap() - prev_delta
    }

    let answer: i64 = input.iter().map(|a| extrapolate(a)).sum();
    println!("{answer}");
}
