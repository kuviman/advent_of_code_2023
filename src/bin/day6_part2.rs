use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day6").unwrap();
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_ascii_whitespace()
        .join("")
        .parse()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_ascii_whitespace()
        .join("")
        .parse()
        .unwrap();

    struct Race {
        time: u64,
        distance: u64,
    }

    let input = Race { time, distance };

    fn solve_brute(race: &Race) -> u64 {
        let mut wins = 0;
        for t in 0..=race.time {
            let speed = t;
            let time_left = race.time - t;
            let our_distance = time_left * speed;
            if our_distance > race.distance {
                wins += 1;
            }
        }
        wins
    }

    fn solve_using_math(race: &Race) -> u64 {
        // (time - t) * t > distance
        //   -t ^ 2 + time * t - distance > 0

        // 0 <= t <= time

        let a = -1_f64;
        let b = race.time as f64;
        let c = -(race.distance as f64);

        // (-b +- sqrt(d)) / 2a
        // d = b^2 - 4ac

        let d = b * b - 4.0 * a * c;
        if d < 0.0 {
            return 0;
        }

        let left = (-b - d.sqrt()) / (2.0 * a);
        let right = (-b + d.sqrt()) / (2.0 * a);

        let (left, right) = (left.min(right), left.max(right));

        const EPS: f64 = 1e-9;

        let left = (left + EPS).floor() as i64 + 1;
        let right = (right - EPS).ceil() as i64 - 1;

        let min_t = left.max(0);
        let max_t = right.min(race.time as i64);

        (max_t + 1 - min_t).max(0) as u64
    }

    let answer: u64 = solve_using_math(&input);
    println!("{answer}");
}
